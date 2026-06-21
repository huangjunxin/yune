import type { TypeDuckBindings } from "./module.js";

export interface TypeDuckCandidate {
  text: string;
  comment: string;
  source?: string;
  quality?: number;
  preedit?: string;
  ai_confidence?: number;
}

export interface TypeDuckInspectorSegment {
  start: number;
  end: number;
  tag: string;
  source: string;
}

export interface TypeDuckFilterAuditRecord {
  name: string;
  before_count: number;
  after_count: number;
}

export interface TypeDuckSpellingAlgebraDebug {
  translator: string;
  input: string;
  lookup_code: string | null;
  formulas: string[];
  expanded_codes: string[];
}

export interface TypeDuckPredictionCandidateDebug {
  index: number;
  text: string;
  source: string;
  quality: number;
  threshold: number | null;
  above_threshold: boolean | null;
}

export interface TypeDuckInspectorDebug {
  segment_tags: string[];
  segments: TypeDuckInspectorSegment[];
  filter_pipeline: string[];
  filter_audit: TypeDuckFilterAuditRecord[];
  spelling_algebra: TypeDuckSpellingAlgebraDebug[];
  prediction: {
    weight_threshold: number | null;
    candidates: TypeDuckPredictionCandidateDebug[];
  };
  ai_staging: {
    state: string;
    for_input: string | null;
  };
}

export interface TypeDuckContext {
  input: string;
  preedit: string;
  caret: number;
  highlighted: number;
  page_size: number;
  page_no: number;
  is_last_page: boolean;
  select_keys: string | null;
  select_labels: string[];
  candidates: TypeDuckCandidate[];
  debug?: TypeDuckInspectorDebug;
}

export interface TypeDuckStatus {
  schema_id: string;
  schema_name: string;
  is_disabled: boolean;
  is_composing: boolean;
  is_ascii_mode: boolean;
  is_full_shape: boolean;
  is_simplified: boolean;
  is_traditional: boolean;
  is_ascii_punct: boolean;
}

export interface TypeDuckResponse {
  handled: boolean;
  commits: string[];
  context: TypeDuckContext | null;
  status: TypeDuckStatus | null;
  error?: string;
}

export class TypeDuckResponseError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "TypeDuckResponseError";
  }
}

export function readTypeDuckResponse(
  responsePtr: number,
  bindings: TypeDuckBindings,
): TypeDuckResponse {
  if (responsePtr === 0) {
    throw new TypeDuckResponseError("TypeDuck adapter returned null response");
  }

  try {
    const jsonPtr = bindings.responseJson(responsePtr);
    if (jsonPtr === 0) {
      throw new TypeDuckResponseError("TypeDuck adapter returned null response JSON");
    }

    const text = bindings.module.UTF8ToString(jsonPtr);
    const parsed = parseResponseJson(text);
    const response = parseTypeDuckResponse(parsed);
    response.handled = bindings.responseHandled(responsePtr) !== 0;
    return response;
  } finally {
    bindings.freeResponse(responsePtr);
  }
}

function parseResponseJson(text: string): unknown {
  try {
    return JSON.parse(text) as unknown;
  } catch {
    throw new TypeDuckResponseError("TypeDuck adapter returned malformed response JSON");
  }
}

function parseTypeDuckResponse(value: unknown): TypeDuckResponse {
  const object = expectRecord(value, "TypeDuck response must be an object");
  if (typeof object.handled !== "boolean") {
    throw new TypeDuckResponseError("TypeDuck response handled field must be boolean");
  }
  if (!Array.isArray(object.commits) || !object.commits.every((commit) => typeof commit === "string")) {
    throw new TypeDuckResponseError("TypeDuck response commits field must be a string array");
  }

  const response: TypeDuckResponse = {
    handled: object.handled,
    commits: object.commits,
    context: parseNullable(object.context, parseTypeDuckContext, "TypeDuck response context field is required"),
    status: parseNullable(object.status, parseTypeDuckStatus, "TypeDuck response status field is required"),
  };

  if (object.error !== undefined) {
    if (typeof object.error !== "string") {
      throw new TypeDuckResponseError("TypeDuck response error field must be a string");
    }
    response.error = object.error;
  }

  return response;
}

function parseTypeDuckContext(value: unknown): TypeDuckContext {
  const object = expectRecord(value, "TypeDuck context must be an object");
  const context: TypeDuckContext = {
    input: expectString(object.input, "TypeDuck context input must be a string"),
    preedit: expectString(object.preedit, "TypeDuck context preedit must be a string"),
    caret: expectNumber(object.caret, "TypeDuck context caret must be a number"),
    highlighted: expectNumber(object.highlighted, "TypeDuck context highlighted must be a number"),
    page_size: expectNumber(object.page_size, "TypeDuck context page_size must be a number"),
    page_no: expectNumber(object.page_no, "TypeDuck context page_no must be a number"),
    is_last_page: expectBoolean(object.is_last_page, "TypeDuck context is_last_page must be boolean"),
    select_keys: parseNullable(object.select_keys, (item) => expectString(item, "TypeDuck context select_keys must be a string"), "TypeDuck context select_keys field is required"),
    select_labels: expectStringArray(object.select_labels, "TypeDuck context select_labels must be a string array"),
    candidates: parseCandidates(object.candidates),
  };
  const debug = parseOptional(object.debug, parseInspectorDebug);
  if (debug !== undefined) {
    context.debug = debug;
  }
  return context;
}

function parseTypeDuckStatus(value: unknown): TypeDuckStatus {
  const object = expectRecord(value, "TypeDuck status must be an object");
  return {
    schema_id: expectString(object.schema_id, "TypeDuck status schema_id must be a string"),
    schema_name: expectString(object.schema_name, "TypeDuck status schema_name must be a string"),
    is_disabled: expectBoolean(object.is_disabled, "TypeDuck status is_disabled must be boolean"),
    is_composing: expectBoolean(object.is_composing, "TypeDuck status is_composing must be boolean"),
    is_ascii_mode: expectBoolean(object.is_ascii_mode, "TypeDuck status is_ascii_mode must be boolean"),
    is_full_shape: expectBoolean(object.is_full_shape, "TypeDuck status is_full_shape must be boolean"),
    is_simplified: expectBoolean(object.is_simplified, "TypeDuck status is_simplified must be boolean"),
    is_traditional: expectBoolean(object.is_traditional, "TypeDuck status is_traditional must be boolean"),
    is_ascii_punct: expectBoolean(object.is_ascii_punct, "TypeDuck status is_ascii_punct must be boolean"),
  };
}

function parseCandidates(value: unknown): TypeDuckCandidate[] {
  if (!Array.isArray(value)) {
    throw new TypeDuckResponseError("TypeDuck context candidates must be an array");
  }
  return value.map((candidate) => {
    const object = expectRecord(candidate, "TypeDuck candidate must be an object");
    const parsed: TypeDuckCandidate = {
      text: expectString(object.text, "TypeDuck candidate text must be a string"),
      comment: expectString(object.comment, "TypeDuck candidate comment must be a string"),
    };
    if (object.source !== undefined && object.source !== null) {
      parsed.source = expectString(object.source, "TypeDuck candidate source must be a string");
    }
    if (object.quality !== undefined && object.quality !== null) {
      parsed.quality = expectNumber(object.quality, "TypeDuck candidate quality must be a number");
    }
    if (object.preedit !== undefined && object.preedit !== null) {
      parsed.preedit = expectString(object.preedit, "TypeDuck candidate preedit must be a string");
    }
    if (object.ai_confidence !== undefined && object.ai_confidence !== null) {
      parsed.ai_confidence = expectNumber(object.ai_confidence, "TypeDuck candidate ai_confidence must be a number");
    }
    return parsed;
  });
}

function parseInspectorDebug(value: unknown): TypeDuckInspectorDebug {
  const object = expectRecord(value, "TypeDuck inspector debug must be an object");
  const prediction = expectRecord(object.prediction, "TypeDuck inspector prediction must be an object");
  const aiStaging = expectRecord(object.ai_staging, "TypeDuck inspector AI staging must be an object");
  return {
    segment_tags: expectStringArray(object.segment_tags, "TypeDuck inspector segment_tags must be a string array"),
    segments: parseArray(object.segments, parseInspectorSegment, "TypeDuck inspector segments must be an array"),
    filter_pipeline: expectStringArray(object.filter_pipeline, "TypeDuck inspector filter_pipeline must be a string array"),
    filter_audit: parseArray(object.filter_audit, parseFilterAuditRecord, "TypeDuck inspector filter_audit must be an array"),
    spelling_algebra: parseArray(object.spelling_algebra, parseSpellingAlgebraDebug, "TypeDuck inspector spelling_algebra must be an array"),
    prediction: {
      weight_threshold: parseNullable(prediction.weight_threshold, (item) => expectNumber(item, "TypeDuck inspector prediction weight_threshold must be a number"), "TypeDuck inspector prediction weight_threshold field is required"),
      candidates: parseArray(prediction.candidates, parsePredictionCandidateDebug, "TypeDuck inspector prediction candidates must be an array"),
    },
    ai_staging: {
      state: expectString(aiStaging.state, "TypeDuck inspector AI staging state must be a string"),
      for_input: parseNullable(aiStaging.for_input, (item) => expectString(item, "TypeDuck inspector AI staging for_input must be a string"), "TypeDuck inspector AI staging for_input field is required"),
    },
  };
}

function parseInspectorSegment(value: unknown): TypeDuckInspectorSegment {
  const object = expectRecord(value, "TypeDuck inspector segment must be an object");
  return {
    start: expectNumber(object.start, "TypeDuck inspector segment start must be a number"),
    end: expectNumber(object.end, "TypeDuck inspector segment end must be a number"),
    tag: expectString(object.tag, "TypeDuck inspector segment tag must be a string"),
    source: expectString(object.source, "TypeDuck inspector segment source must be a string"),
  };
}

function parseFilterAuditRecord(value: unknown): TypeDuckFilterAuditRecord {
  const object = expectRecord(value, "TypeDuck inspector filter audit record must be an object");
  return {
    name: expectString(object.name, "TypeDuck inspector filter audit name must be a string"),
    before_count: expectNumber(object.before_count, "TypeDuck inspector filter audit before_count must be a number"),
    after_count: expectNumber(object.after_count, "TypeDuck inspector filter audit after_count must be a number"),
  };
}

function parseSpellingAlgebraDebug(value: unknown): TypeDuckSpellingAlgebraDebug {
  const object = expectRecord(value, "TypeDuck inspector spelling algebra must be an object");
  return {
    translator: expectString(object.translator, "TypeDuck inspector spelling algebra translator must be a string"),
    input: expectString(object.input, "TypeDuck inspector spelling algebra input must be a string"),
    lookup_code: parseNullable(object.lookup_code, (item) => expectString(item, "TypeDuck inspector spelling algebra lookup_code must be a string"), "TypeDuck inspector spelling algebra lookup_code field is required"),
    formulas: expectStringArray(object.formulas, "TypeDuck inspector spelling algebra formulas must be a string array"),
    expanded_codes: expectStringArray(object.expanded_codes, "TypeDuck inspector spelling algebra expanded_codes must be a string array"),
  };
}

function parsePredictionCandidateDebug(value: unknown): TypeDuckPredictionCandidateDebug {
  const object = expectRecord(value, "TypeDuck inspector prediction candidate must be an object");
  return {
    index: expectNumber(object.index, "TypeDuck inspector prediction candidate index must be a number"),
    text: expectString(object.text, "TypeDuck inspector prediction candidate text must be a string"),
    source: expectString(object.source, "TypeDuck inspector prediction candidate source must be a string"),
    quality: expectNumber(object.quality, "TypeDuck inspector prediction candidate quality must be a number"),
    threshold: parseNullable(object.threshold, (item) => expectNumber(item, "TypeDuck inspector prediction candidate threshold must be a number"), "TypeDuck inspector prediction candidate threshold field is required"),
    above_threshold: parseNullable(object.above_threshold, (item) => expectBoolean(item, "TypeDuck inspector prediction candidate above_threshold must be boolean"), "TypeDuck inspector prediction candidate above_threshold field is required"),
  };
}

function parseOptional<T>(
  value: unknown,
  parser: (value: unknown) => T,
): T | undefined {
  if (value === undefined || value === null) {
    return undefined;
  }
  return parser(value);
}

function parseNullable<T>(
  value: unknown,
  parser: (value: unknown) => T,
  missingMessage: string,
): T | null {
  if (value === undefined) {
    throw new TypeDuckResponseError(missingMessage);
  }
  if (value === null) {
    return null;
  }
  return parser(value);
}

function parseArray<T>(
  value: unknown,
  parser: (value: unknown) => T,
  message: string,
): T[] {
  if (!Array.isArray(value)) {
    throw new TypeDuckResponseError(message);
  }
  return value.map(parser);
}

function expectRecord(value: unknown, message: string): Record<string, unknown> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    throw new TypeDuckResponseError(message);
  }
  return value as Record<string, unknown>;
}

function expectString(value: unknown, message: string): string {
  if (typeof value !== "string") {
    throw new TypeDuckResponseError(message);
  }
  return value;
}

function expectStringArray(value: unknown, message: string): string[] {
  if (!Array.isArray(value) || !value.every((item) => typeof item === "string")) {
    throw new TypeDuckResponseError(message);
  }
  return value;
}

function expectNumber(value: unknown, message: string): number {
  if (typeof value !== "number" || !Number.isFinite(value)) {
    throw new TypeDuckResponseError(message);
  }
  return value;
}

function expectBoolean(value: unknown, message: string): boolean {
  if (typeof value !== "boolean") {
    throw new TypeDuckResponseError(message);
  }
  return value;
}

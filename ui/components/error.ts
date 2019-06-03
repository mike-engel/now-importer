import { Maybe } from "true-myth";
import { htm } from "@zeit/integration-utils";

export const ImportError = ({ error }: { error: Maybe<string> }) =>
  error.mapOr("", err => htm`<Notice type="error">${err}</Notice>`);

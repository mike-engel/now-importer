import { Maybe } from "true-myth";
import { Deployment } from "./zeit";

export type Store = {
  deployUrl: Maybe<string>;
  error: Maybe<string>;
  importedDeployments: Deployment[];
};

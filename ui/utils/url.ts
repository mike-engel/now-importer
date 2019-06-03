import { Maybe, Result } from "true-myth";
import normalizeUrl from "normalize-url";

export const verifyUrl = (url: Maybe<string>): Result<string, string> =>
  url.mapOr(Result.err("A URL must be provided"), unwrappedUrl => {
    try {
      const parsedUrl = normalizeUrl(unwrappedUrl);

      return Result.ok(parsedUrl);
    } catch (_) {
      return Result.err("A URL must be provided");
    }
  });

export const extractProjectName = (url: string, username: string) => {
  return url.replace(`.${username}.now.sh`, "");
};

export const extractDeployId = (url: string, deployName: string) => {
  const re = new RegExp(`${deployName}-([A-z0-9]+)\.now\.sh`);
  const matches = url.match(re);

  if (!matches || !matches.length) return "???";

  return matches[1];
};

export const fqdn = (url: string) => normalizeUrl(url, { forceHttps: true });

export const projectUrl = (url: string, username: string) =>
  `https://zeit.co/${username}/${extractProjectName(url, username)}`;

export const deploymentUrl = (url: string, name: string, username: string) =>
  `https://zeit.co/${username}/${name}/${extractDeployId(url, name)}`;

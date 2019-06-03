import { withUiHook, htm, HandlerOptions } from "@zeit/integration-utils";
import { Maybe } from "true-myth";
import { verifyUrl } from "./utils/url";
import { importWebsite } from "./utils/import";
import { Store } from "./types/store";
import { Actions } from "./types/actions";
import { ImportSuccess } from "./components/success";
import { NewImport } from "./components/new_import";
import { RecentDeployments } from "./components/recent_deployments";

const store: Store = {
  deployUrl: Maybe.nothing(),
  error: Maybe.nothing(),
  importedDeployments: []
};

export const executeAction = async ({ payload, zeitClient }: HandlerOptions) => {
  switch (payload.action) {
    case Actions.ImportUrl:
      const url = verifyUrl(Maybe.of(payload.clientState.url));

      await url.match<void>({
        Ok: async importUrl => {
          try {
            const url = await importWebsite(importUrl, zeitClient.options.token);

            store.error = Maybe.nothing();
            store.deployUrl = Maybe.just(url);
          } catch (err) {
            store.deployUrl = Maybe.nothing();
            store.error = Maybe.just(`Error importing your website: ${err.message}`);
          }
        },
        Err: err => (store.error = Maybe.just(err))
      });

      return;
    case Actions.StartOver:
      store.deployUrl = Maybe.nothing();
      store.error = Maybe.nothing();

      return;
    default:
      return;
  }
};

const getPreviousImports = async (client: HandlerOptions["zeitClient"]) => {
  const { deployments } = await client.fetchAndThrow("/v4/now/deployments?meta-imported=true", {});

  return deployments;
};

export default withUiHook(async (options: HandlerOptions) => {
  await executeAction(options);

  store.importedDeployments = await getPreviousImports(options.zeitClient);

  return htm`
    <Page>
      <Box display="flex" justifyContent="space-between" alignItems="flex-start">
        <Box flex="2 66%" paddingRight="36px" borderRight="1px solid rgb(183, 183, 183)">
          ${store.deployUrl.match({
            Just: url => htm`<${ImportSuccess} url=${url} user=${options.payload.user} />`,
            Nothing: () => htm`<${NewImport} store=${store} />`
          })}
        </Box>
        <Box flex="1 33%" marginLeft="36px">
          <${RecentDeployments} deployments=${store.importedDeployments} user=${
    options.payload.user
  } />
        </Box>
      </Box>
    </Page>
  `;
});

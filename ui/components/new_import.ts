import { htm } from "@zeit/integration-utils";
import { Actions } from "../types/actions";
import { ImportError } from "./error";
import { Store } from "../types/store";

type Props = {
	store: Store;
};

export const NewImport = ({ store }: Props) => htm`
  <H1>Now importer</H1>
  <P>Place the URL to your existing website below and click Import to download and deploy your website with now!</P>
  <Box border="1px solid rgb(183, 183, 183)" borderRadius="2px" padding="8px 16px">
    <P>
      <B>NOTE:</B> Currently, only static websites are supported. If your site uses a custom server like PHP, Node, Ruby on Rails, etc., this will not work 😞
    </P>
  </Box>
  <${ImportError} error=${store.error} />
  <Fieldset>
    <FsContent>
      <Input name="url" label="Your website's URL" type="url" width="100%" placeholder="http://yoursite.com" value="" />
    </FsContent>
  </Fieldset>
  <Box display="flex" justifyContent="flex-end">
    <Button action=${Actions.ImportUrl}>Import</Button>
  </Box>
`;

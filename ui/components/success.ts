import { htm, HandlerOptions } from "@zeit/integration-utils";
import { fqdn, projectUrl } from "../utils/url";
import { Actions } from "../types/actions";

type Props = {
	url: string;
	user: HandlerOptions["payload"]["user"];
};

export const ImportSuccess = ({ url, user }: Props) =>
	htm`
    <H1>Success!</H1>
    <P><B>Your website is live on now! Open <Link href=${fqdn(
			url
		)}>${url}</Link> to view your website.</B></P>
    <P>
      Visit the <Link href=${projectUrl(
				url,
				user.username
			)}> project page</Link> to view your deployment, manage aliases, and more.
    </P>
    <Box borderTop="1px solid rgb(183, 183, 183)" paddingTop="8px" marginTop="24px">
      <P>New to the now platform? You can read the <Link href="https://zeit.co/docs">official docs</Link> to learn about now, deployments, aliases, and more.</P>
      <Button action=${Actions.StartOver}>Start over</Button>
    </Box>
  `;

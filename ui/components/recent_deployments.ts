import { Deployment } from "../types/zeit";
import { htm, HandlerOptions } from "@zeit/integration-utils";
import { deploymentUrl, extractDeployId } from "../utils/url";

type Props = {
	deployments: Deployment[];
	user: HandlerOptions["payload"]["user"];
};

export const RecentDeployments = ({ deployments, user }: Props) => {
	if (!deployments.length) {
		return htm`
      <H2>Recent imports</H2>
      <P>No recent imports (yet)!</P>
    `;
	}

	return htm`
    <H2>Recent imports</H2>
    <UL>
      ${deployments.map(
				deployment =>
					htm`<LI><Link href=${deploymentUrl(
						deployment.url,
						deployment.name,
						user.username
					)}>${deployment.name}</Link> ${"(" +
						extractDeployId(deployment.url, deployment.name) +
						")"}</LI>`
			)}
    </UL>
  `;
};

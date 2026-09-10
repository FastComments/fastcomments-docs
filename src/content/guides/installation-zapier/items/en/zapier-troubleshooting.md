## Troubleshooting

**"You do not have permission" when connecting.** The signed-in user is not an API admin on the account.
Ask the account owner to grant the API permission on the Users page, or connect as the owner.

**The connection is labelled with the wrong site.** The consent page connects the account you were signed
in to at the time. Disconnect in Zapier, switch accounts in the FastComments dashboard, and connect again.

**Events stopped arriving.** Check the Webhooks page in the dashboard. A subscription whose endpoint kept
failing for six days is disabled automatically and shows why. Re-enable it there, or turn the Zap off and on
again. If the subscription is missing entirely, someone deleted it; turning the Zap off and on recreates it.

**Zapier says the account needs to be reconnected.** The connection was revoked from the Connected Apps
page, the user who approved it lost the API permission, or the account was deleted. Reconnect from Zapier.

**An action fails with "does not have write access".** The connection was approved with read-only
permission. Reconnect and approve both permissions.

**Rate limits and credits.** Actions and searches spend API credits from your plan and are subject to the
same rate limits as the REST API. Triggers spend none. A Zap that runs into a limit is retried by Zapier
after the delay FastComments reports.

**The Domain dropdown is empty.** Domains appear once they are configured on the Domains page in the
FastComments dashboard. Leave the field blank to receive events for every domain.

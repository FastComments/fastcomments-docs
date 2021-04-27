For Developers whom you may not want to be `Administrators`, consider creating an `Administrator` user
with the following permissions:

1. Analytics Admin
2. Customizations Admin
3. Data Management Admin
4. Comment Moderation Admin
5. API/SSO Admin

This set of permissions will give a developer everything they need to setup FastComments as well
as the visibility into the system required to ensure it is working.

The reasoning for these permissions is as follows:

1. **Analytics Admin**: This can be used to debug usage of FastComments.
2. **Customizations Admin**: This will be required to apply custom styling to the comment widget.
3. **Data Management Admin**: This will be required to manage imports and exports, and setup webhooks.
4. **Comment Moderation Admin**: This will be required to see comment data, at least during setup.
5. **API/SSO Admin**: This will allow them to fetch the API keys directly from our platform. We consider
this more secure than an `Administrator` copying it for them and sending the API Secret via email which
   may not be very secure.

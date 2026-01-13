It is common to have a sub tenant per test or dev environment with FastComments. Each tenant would have its own configuration, data, and API keys. Configuration, data, and users cannot be shared across tenants.
Everything is isolated. However, super admins of the parent tenant can impersonate users in child tenants.

There are two approaches:

- The main tenant is for production, and sub-tenants are for test environments.
- The main tenant is simply for billing, and each sub-tenant is for prod, test, and so on.

The first is generally easier for users to reason about, but this may depend on your organization.

Tenants can be created [here](https://eu.fastcomments.com/auth/my-account/tenants) if you have the package. This is also where super admins would
impersonate users. Tenants can also be created via the API for more custom/automated setups.

No matter the approach taken, you'll have to add the moderators and users that want to see production data in the "prod" tenant. So for example if you want
to go with option B and have the parent tenant for billing, and have a sub tenant for "prod", you'll want to add the tenant, switch to the new tenant, and add your
admin and moderator users for the sub-tenant. 

Finally, to clarify, the Moderate Comments page will be empty with option B for the parent tenant.

---
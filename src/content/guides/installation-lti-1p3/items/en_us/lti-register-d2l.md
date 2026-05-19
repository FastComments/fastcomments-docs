D2L Brightspace exposes Dynamic Registration through the LTI Advantage admin interface. You will need administrator access.

#### Open the Registration Screen

1. Sign in to your Brightspace instance as an admin.
2. Navigate to **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Click **Register Tool**. (The direct URL is `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Paste the URL

You'll see a registration form. The key field is **Tool initiation registration endpoint** (some Brightspace versions label it "Tool Initiation Registration URL").

Paste the FastComments registration URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">get it here</a>) into that field. Leave the other fields blank - they're auto-populated by FastComments during the registration handshake.

Click **Register**.

#### Approve the Tool

Brightspace opens a popup that talks to FastComments, exchanges keys, and shows a confirmation screen. The popup closes itself when registration completes.

The new tool appears in your LTI Advantage tool list. By default Brightspace marks new tools as **disabled** - flip the toggle to **enabled** so your courses can use it.

#### Add a Deployment

In Brightspace, LTI tools need a **deployment** before they can be used in courses:

1. Open the newly-registered FastComments tool.
2. Click **View Deployments** > **New Deployment**.
3. Give the deployment a name (e.g. "FastComments - All Courses"), pick the org units it should be available in, and save.

After the first launch through this deployment, FastComments pins the `deployment_id` to its configuration record - subsequent launches from a different deployment under the same client will be rejected unless you re-register.

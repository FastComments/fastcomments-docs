#### Navigate to LTI 1.3 Configuration

Sign in to FastComments and go to <a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">your LTI 1.3 Configuration page</a>.

If your account doesn't yet have LTI access, you'll see "LTI not enabled for this account" - contact support to enable it on your plan.

#### Pick a Platform (Optional)

Under **Generate a Dynamic Registration URL**, use the **Platform** dropdown to tell FastComments which LMS you're connecting:

- D2L Brightspace
- Moodle
- Blackboard Learn
- Sakai
- Schoology
- Other LTI 1.3 platform

You can also leave it on **Auto-detect**. The platform is read out of your LMS's openid-configuration during registration; the dropdown only seeds the display label for the resulting configuration.

#### Generate the URL

Click **Generate URL**. FastComments creates a one-time registration token and shows you a URL that looks like:

`https://fastcomments.com/lti/v1p3/register/<long-token>`

Copy it. This URL:

- Is **single-use** - once your LMS calls it successfully, the token is consumed.
- Expires after **30 minutes** if not used.
- Should be kept private - anyone with the URL can register a tool against your tenant within those 30 minutes.

#### Existing Configurations

Once a registration completes successfully, the new configuration shows up in the **Existing Configurations** table on the same page, with its Platform, Issuer, Client ID, and Status. You can delete configurations from this table if you ever need to unregister.

**Using Moodle?** We also publish a dedicated Moodle plugin for FastComments with a tighter integration than LTI 1.3 (grade sync hooks, deeper activity reporting, native Moodle settings UI). See the <a href="/guide-installation-moodle.html" target="_blank">Moodle plugin installation guide</a>. The LTI 1.3 flow below is the right choice if you want a single registration that also covers other LMSes, or if your Moodle admin won't install third-party plugins.

Moodle 4.0+ supports LTI 1.3 Dynamic Registration through the External Tool plugin.

#### Open the Tool Management Screen

1. Sign in to Moodle as a site administrator.
2. Navigate to **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools**.

#### Paste the URL

You'll see a card labeled **Tool URL**. Paste the FastComments registration URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">get it here</a>) into the text field and click **Add LTI Advantage**.

Moodle opens a registration screen showing the tool's identity and the permissions it's requesting. Review and click **Activate** (or **Register**, depending on Moodle version).

The popup closes when registration completes; the new FastComments tool appears in the **Tools** list with the status **Active**.

#### Make It Available

By default Moodle adds new tools to the "Course tools" list but doesn't show them in the activity picker. To expose FastComments course-wide:

1. Click the gear icon on the FastComments tile.
2. Under **Tool configuration usage**, choose **Show in activity chooser and as a preconfigured tool**.
3. Save.

Instructors can now add FastComments to any course through **Add an activity or resource** > **FastComments**.

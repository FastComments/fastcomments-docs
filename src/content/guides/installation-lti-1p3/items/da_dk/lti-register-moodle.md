**Using Moodle?** Vi udgiver også et dedikeret Moodle-plugin for FastComments med en tættere integration end LTI 1.3 (grade sync hooks, dybere aktivitetsrapportering, indbygget Moodle-indstillings-UI). Se <a href="/guide-installation-moodle.html" target="_blank">Moodle plugin installation guide</a>. LTI 1.3-flowet nedenfor er det rigtige valg, hvis du ønsker en enkelt registrering, der også dækker andre LMS'er, eller hvis din Moodle-admin ikke vil installere tredjeparts-plugins.

Moodle 4.0+ understøtter LTI 1.3 Dynamic Registration gennem External Tool-plugin'et.

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
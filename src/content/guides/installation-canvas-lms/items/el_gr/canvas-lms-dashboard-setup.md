#### Navigate to Canvas LTI Config

Log in to your FastComments account and go to <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">My Account > Canvas LTI Config</a>.

#### Create a New LTI Configuration

Check the **Enable LTI** checkbox. The configuration fields will appear:

- **Configuration Name** - μια προαιρετική ετικέτα για να αναγνωρίσετε αυτήν τη διαμόρφωση (χρήσιμη αν συνδέσετε πολλαπλές Canvas instances).
- **Platform URL** - το URL της εγκατάστασης Canvas σας (π.χ. `https://yourschool.instructure.com`). Μπορείτε να το αφήσετε κενό προς το παρόν και να το συμπληρώσετε μετά τη δημιουργία του Developer Key.
- **Client ID** - αφήστε το κενό προς το παρόν. Θα το συμπληρώσετε μετά τη δημιουργία του Developer Key στο Canvas.
- **Deployment ID** - αφήστε το κενό προς το παρόν.
- **Comment Style** - επιλέξτε ανάμεσα σε Comments, Collab Chat, ή Both. Δείτε τα [Commenting Styles](#canvas-lms-commenting-styles) για λεπτομέρειες.

Click **Add** to create the configuration.

#### Copy the Configuration URLs

After saving, three URLs will appear:

- **Configuration URL** - you will paste this into Canvas when creating the Developer Key.
- **OIDC Login URL** - used by Canvas for the LTI login flow (automatically configured via the Configuration URL).
- **Launch URL** - the endpoint Canvas calls when a student opens FastComments (automatically configured via the Configuration URL).

Copy the **Configuration URL**. You will need it in the next step.
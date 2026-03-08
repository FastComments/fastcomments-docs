#### Navigate to Canvas LTI Config

Log in to your FastComments account and go to <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">我的帳戶 &gt; Canvas LTI 設定</a>.

#### Create a New LTI Configuration

Check the **Enable LTI** checkbox. The configuration fields will appear:

- **Configuration Name** - an optional label to identify this configuration (useful if you connect multiple Canvas instances).
- **Platform URL** - your Canvas instance URL (e.g. `https://yourschool.instructure.com`). You can leave this blank for now and fill it in after creating the Developer Key.
- **Client ID** - leave this blank for now. You will fill it in after creating the Developer Key in Canvas.
- **Deployment ID** - leave this blank for now.
- **Comment Style** - choose between 評論、協作聊天，或兩者。See [Commenting Styles](#canvas-lms-commenting-styles) for details.

Click **Add** to create the configuration.

#### Copy the Configuration URLs

After saving, three URLs will appear:

- **Configuration URL** - you will paste this into Canvas when creating the Developer Key.
- **OIDC Login URL** - used by Canvas for the LTI login flow (automatically configured via the Configuration URL).
- **Launch URL** - the endpoint Canvas calls when a student opens FastComments (automatically configured via the Configuration URL).

Copy the **Configuration URL**. You will need it in the next step.
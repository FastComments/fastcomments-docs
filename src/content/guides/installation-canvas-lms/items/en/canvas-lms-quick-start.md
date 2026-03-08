1. Log in to FastComments and go to <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">My Account > Canvas LTI Config</a>.
2. Enter a **Configuration Name** and your **Platform URL** (e.g. `https://yourschool.instructure.com`). Choose which **Placements** to enable (Assignment View and/or Editor Button — both are on by default). Click **Create Configuration**. The wizard advances to Step 2 and shows your **Configuration URL**.
3. In Canvas, go to **Admin > Developer Keys > + Developer Key > LTI Key**. Set **Method** to "Enter URL" and paste the Configuration URL. Save the key, then set its **State** to **ON** and click **Allow** when prompted.
4. Copy the **Client ID** number from the Developer Keys table in Canvas. Back in FastComments, paste it into the **Client ID** field and click **Save & Continue**.
5. Review the configuration summary and click **Enable Integration** to go live.
6. Install the External App in Canvas (**Admin > Settings > Apps > + App > By Client ID**). Comments will automatically appear below assignments, and instructors can embed FastComments in Pages, Quizzes, and Announcements via the Rich Content Editor toolbar button.

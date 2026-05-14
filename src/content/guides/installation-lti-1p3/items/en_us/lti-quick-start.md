1. Sign in to FastComments and go to <a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">your LTI 1.3 Configuration page</a>.
2. (Optional) Pick the platform you're connecting from the **Platform** dropdown - it sets the display label, but Auto-detect works fine.
3. Click **Generate URL**. A one-time **Registration URL** appears (valid for 30 minutes, single-use).
4. In your LMS, open the LTI 1.3 Dynamic Registration screen and paste the URL into the **Tool initiation registration endpoint** (or equivalent) field. Submit.
5. Your LMS calls back to FastComments, exchanges keys, and creates the integration. The popup closes itself when done.
6. Back in FastComments, the new configuration appears in the **Existing Configurations** table. The tool is now available inside your LMS courses.

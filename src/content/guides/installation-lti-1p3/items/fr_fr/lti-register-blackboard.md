Blackboard Learn SaaS and Ultra support LTI 1.3 Dynamic Registration.

#### Open the Tool Provider Screen

1. Connectez-vous à Blackboard en tant qu'administrateur système.
2. Navigate to **Administrator Panel** > **Integrations** > **LTI Tool Providers**.
3. Click **Register LTI 1.3 / LTI Advantage Tool**.

If you only see "Register LTI 1.1 Provider", your Blackboard version doesn't support LTI 1.3 yet - upgrade or contact Blackboard support.

#### Paste the URL

Paste the FastComments registration URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">obtenez-la ici</a>) into the **Client ID** / **Registration URL** field (Blackboard's labeling varies by version). Submit.

Blackboard performs the registration handshake with FastComments and shows you a confirmation screen.

#### Approve and Enable

Blackboard marks newly-registered tools as **Approved but excluded** by default:

1. Find the FastComments entry in the tool provider list.
2. Open the menu and choose **Edit**.
3. Set **Tool Status** to **Approved**.
4. Under **Institution Policies**, review what user data is sent (name, email, role). Save.

The tool is now available to instructors when they add content to courses.
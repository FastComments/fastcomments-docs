#### "Registration token not found, expired, or already used"

The token in your registration URL is valid for 30 minutes and can only be used once. If your LMS took longer than that, or if the registration was retried after succeeding, the token will be rejected. Generate a fresh URL in the FastComments LTI 1.3 Configuration page and start over.

#### "Platform rejected registration"

Your LMS refused the registration handshake. The most common causes:

- **Tool already registered with the same client name.** Some platforms (notably D2L) reject a second registration of "FastComments" until the previous one is deleted. Remove the old tool in your LMS, then retry.
- **Wrong field in the LMS.** Make sure you pasted the URL into the **registration / tool initiation registration endpoint** field, not the launch URL or login URL field.
- **The LMS doesn't actually support Dynamic Registration.** Older Moodle and Blackboard versions advertise LTI 1.3 but only allow manual configuration. Check your platform's docs.

#### "Failed to fetch platform configuration"

FastComments was unable to read your LMS's openid-configuration document. This is rare and usually means the LMS provided a malformed or unreachable discovery URL. Contact your LMS support.

#### Launch shows "Configuration not found"

Either the configuration in FastComments was deleted, or the launch came from an `iss`/`client_id` pair we don't recognize. If you deleted and re-registered, instruct your LMS to remove and re-add the FastComments tool so it gets the new client_id.

#### Launch shows "Deployment not registered"

You launched FastComments from a Brightspace/Moodle/Blackboard deployment different from the one it was first launched in. FastComments pins the `deployment_id` on first launch as a security check. To add a new deployment under the same client, contact support - we'll add the deployment ID to the configuration.

#### Launch shows "Unsupported message_type"

The LMS sent an LTI message FastComments doesn't handle (e.g. `LtiSubmissionReviewRequest`). FastComments supports the standard resource-link launch and deep-linking flows only. Reach out if you need a specific message type added.

#### Iframe doesn't resize

Most LMSes auto-size LTI iframes. If yours doesn't, check that the LMS's launch settings allow the tool to send postMessage events to the parent frame. FastComments emits both Canvas-style (`lti.frameResize`) and IMS-spec (`org.imsglobal.lti.frameResize`) resize messages.

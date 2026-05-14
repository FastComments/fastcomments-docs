#### Sakai

Sakai supports LTI 1.3 Dynamic Registration on releases with LTI Advantage. From the Administration Workspace:

1. Sign in as a Sakai admin and open the **Administration Workspace**.
2. Choose **External Tools** > **Install LTI 1.3 Tool**.
3. Paste the FastComments registration URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">get it here</a>) and submit.
4. Approve the tool when the handshake completes.

The tool then appears under **External Tools** and can be added to sites by their maintainers.

#### Schoology

Schoology Enterprise instances support LTI 1.3 but Dynamic Registration availability varies by deployment. Check with your Schoology account manager.

If Dynamic Registration isn't available on your Schoology instance, you'll need to configure the integration manually using these endpoints:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

After Schoology gives you a Client ID and Deployment ID, contact FastComments support to register the configuration on your tenant.

#### Other LTI 1.3 Platforms

Any LMS that follows the IMS LTI 1.3 Advantage spec should work with the same registration URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">get it here</a>). Look for a setting labeled "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint", or similar.

If your platform only supports manual LTI 1.3 setup, use the four endpoints listed in the Schoology section above and contact support to finalize.

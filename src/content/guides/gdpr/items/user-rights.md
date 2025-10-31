### GDPR User Rights

GDPR grants users several rights regarding their personal data. FastComments provides built-in tools to help you honor these rights for your users.

### Right to Erasure (Right to be Forgotten)

#### What It Is

GDPR Article 17 grants users the right to have their personal data deleted under certain circumstances. Users can request that their data be erased when:

- The data is no longer necessary for the purpose it was collected
- They withdraw consent for data processing
- They object to the processing of their data
- The data has been unlawfully processed

#### How FastComments Supports This

FastComments provides account deletion functionality that allows users to exercise their right to erasure:

**For End Users (Commenters):**
- Users can request deletion of their commenting account
- When an account is deleted, associated personal data is removed from the system
- Comments can be anonymized or deleted based on your configuration

**For FastComments Account Holders:**
- You can delete your FastComments account through your account settings
- Deletion removes your account data and associated information
- See our [Account Deletion Guide](/guide-deleting-account.html) for detailed instructions

#### Implementation in Your Application

To support user deletion requests in your application:

1. **Receive deletion requests** - Provide a way for users to request data deletion
2. **Verify the user's identity** - Ensure the request is legitimate
3. **Delete the user's comments** - Use the FastComments API to remove or anonymize comments
4. **Document the deletion** - Keep records of deletion requests for compliance purposes

### Right to Data Portability

#### What It Is

GDPR Article 20 grants users the right to receive their personal data in a structured, commonly used, and machine-readable format. Users can also request that their data be transmitted directly to another service provider.

#### How FastComments Supports This

FastComments provides data export functionality to help you honor data portability requests:

**Data Export Features:**
- Export user comment history
- Export user profile information
- Export data in structured JSON format
- Machine-readable format suitable for import into other systems

#### Exporting User Data

**Via the API:**

You can use the FastComments API to export user data:

1. **Authenticate** - Use your API credentials
2. **Request user data** - Call the appropriate API endpoint with the user's identifier
3. **Receive structured data** - Data is returned in JSON format
4. **Provide to user** - Deliver the exported data to the user or transfer it to another service

**Data Included in Exports:**

- User profile information (name, email, etc.)
- Comment content and metadata
- Comment timestamps and edit history
- Vote history (if applicable)
- Notification preferences

### Right to Access

#### What It Is

GDPR Article 15 grants users the right to access their personal data and information about how it's being processed.

#### How to Provide Access

Users should be able to:

1. **View their data** - Users can see their comments and profile through the FastComments interface
2. **Request a copy** - Use the data export feature to provide a complete copy of their data
3. **Understand processing** - Refer users to your privacy policy and this documentation

### Right to Rectification

#### What It Is

GDPR Article 16 grants users the right to correct inaccurate or incomplete personal data.

#### How FastComments Supports This

- **Edit comments** - Users can edit their own comments to correct information
- **Update profiles** - Users can update their profile information
- **API support** - You can use the API to update user data on their behalf

### Processing Requests

#### Response Timeline

Under GDPR, you must respond to user rights requests within **one month** of receiving the request. This can be extended by two additional months for complex requests, but you must inform the user of the extension within the first month.

#### Verification

Before processing requests:

1. **Verify identity** - Ensure the request is from the actual user
2. **Clarify the request** - Make sure you understand what the user is asking for
3. **Document everything** - Keep records of the request and your response

#### Free of Charge

In most cases, you must fulfill user rights requests free of charge. You may charge a reasonable fee only if requests are manifestly unfounded, excessive, or repetitive.

### Best Practices

- **Automate when possible** - Build self-service tools for common requests
- **Document processes** - Maintain clear procedures for handling each type of request
- **Train your team** - Ensure staff understand how to handle user rights requests
- **Respond promptly** - Don't wait until the deadline to respond
- **Be transparent** - Clearly communicate what data you have and how it's used
- **Keep records** - Maintain logs of all user rights requests and how they were handled

Esta guía cubre problemas comunes de autenticación SAML y sus soluciones.

### Certificate and Security Issues

#### Invalid Certificate Error

**Symptoms**:
- "Certificate validation failed" error
- Users cannot complete SAML authentication
- SAML responses are rejected

**Common Causes**:
- Certificate format is incorrect
- Certificate has expired
- Wrong certificate was provided
- Extra characters or whitespace in certificate

**Solutions**:
1. **Verify Certificate Format**:
   - Ensure certificate includes `-----BEGIN CERTIFICATE-----` and `-----END CERTIFICATE-----` markers
   - Remove any extra whitespace or line breaks
   - Copy certificate directly from IdP metadata or configuration

2. **Check Certificate Validity**:
   - Verify certificate hasn't expired
   - Confirm certificate is for the correct IdP
   - Use online certificate validators to check format

3. **Re-download Certificate**:
   - Download fresh certificate from IdP
   - Use IdP metadata URL if available
   - Confirm certificate matches current IdP configuration

#### Signature Verification Failed

**Symptoms**:
- SAML assertion signature validation errors
- Authentication fails after IdP login
- "Invalid signature" error messages

**Solutions**:
1. **Algorithm Mismatch**:
   - Check signature algorithm in FastComments matches IdP
   - Try different signature algorithms (SHA-256, SHA-1, SHA-512)
   - Verify digest algorithm matches IdP configuration

2. **Certificate Issues**:
   - Ensure correct signing certificate is configured
   - Verify certificate corresponds to private key used by IdP
   - Check for certificate rotation in IdP

### Configuration Issues

#### Wrong Entity ID or ACS URL

**Symptoms**:
- IdP reports "Unknown Service Provider"
- SAML responses go to wrong endpoint
- Authentication doesn't complete

**Solutions**:
1. **Verify SP Information**:
   - Copy exact Entity ID from FastComments configuration
   - Ensure ACS URL matches format: `https://fastcomments.com/saml/callback/{tenant-id}`
   - Check for typos in tenant ID

2. **IdP Configuration**:
   - Update IdP with correct SP Entity ID
   - Configure proper ACS/Reply URL
   - Verify IdP binding settings (HTTP-POST preferred)

#### Missing or Incorrect Attributes

**Symptoms**:
- Users created without proper roles
- Missing user profile information
- "Email required" errors

**Solutions**:
1. **Email Attribute**:
   - Ensure IdP sends email attribute
   - Check attribute name mapping (email, emailAddress, etc.)
   - Verify email value is valid email address

2. **Role Attributes**:
   - Confirm IdP sends role/group information
   - Check role attribute names match FastComments expectations
   - Verify role values match FastComments role names exactly

3. **Attribute Format**:
   - Test both array and comma-separated role formats
   - Ensure attribute values don't have extra whitespace
   - Check for case sensitivity in role names

### Authentication Flow Issues

#### Redirect Loop

**Symptoms**:
- Browser redirects endlessly between FastComments and IdP
- Authentication never completes
- Multiple redirects shown in browser developer tools

**Solutions**:
1. **Check SP Configuration**:
   - Verify Entity ID matches IdP configuration exactly
   - Ensure ACS URL is correctly configured in IdP
   - Check for trailing slashes in URLs

2. **Session Issues**:
   - Clear browser cookies and try again
   - Test in incognito/private browser window
   - Check for session timeout settings

#### Access Denied After Authentication

**Symptoms**:
- SAML authentication succeeds
- User is redirected to FastComments
- "Access denied" or permissions error displayed

**Solutions**:
1. **Role Assignment**:
   - Verify user has appropriate roles in IdP
   - Check role attribute is being sent in SAML response
   - Confirm role names match FastComments requirements exactly

2. **Package Limitations**:
   - Verify account has Flex or Pro plan
   - Check SAML feature is enabled for the package
   - Contact support if package includes SAML but feature unavailable

### Identity Provider Specific Issues

#### Microsoft Azure AD

**Common Issues**:
- App role assignments not reflecting in tokens
- Claims not being sent properly
- User assignment requirements

**Solutions**:
- Check user assignment to FastComments application
- Verify app roles are properly configured
- Ensure claims mapping includes required attributes

#### Okta

**Common Issues**:
- Group filters not working correctly
- Attribute statements misconfigured
- Application assignment problems

**Solutions**:
- Review attribute statement configuration
- Check group assignment and filtering rules
- Verify application is assigned to appropriate users/groups

#### Google Workspace

**Common Issues**:
- Custom attributes not mapping correctly
- Group membership not being sent
- SAML application configuration errors

**Solutions**:
- Configure custom schema for role attributes
- Check group membership propagation
- Verify SAML application attribute mapping

### Network and Connectivity Issues

#### Timeout Errors

**Symptoms**:
- Authentication process times out
- "Request timeout" or similar errors
- Slow authentication flow

**Solutions**:
1. **Network Connectivity**:
   - Check firewall rules allow FastComments communication
   - Verify DNS resolution for fastcomments.com
   - Test network connectivity from IdP to FastComments

2. **Performance Issues**:
   - Check IdP response times
   - Verify certificate chain validation isn't slow
   - Consider network latency between IdP and users

#### SSL/TLS Issues

**Symptoms**:
- Certificate warnings during authentication
- SSL handshake failures
- "Secure connection failed" errors

**Solutions**:
- Ensure all SAML endpoints use HTTPS
- Check certificate validity for all involved domains
- Verify TLS version compatibility

### Debugging and Logging

#### Enabling Debug Information

1. **Browser Developer Tools**:
   - Monitor Network tab during SAML flow
   - Check Console for JavaScript errors
   - Examine SAML POST requests (if visible)

2. **IdP Logging**:
   - Enable SAML debugging in your IdP
   - Review IdP logs for SAML request/response details
   - Check for attribute mapping issues

#### Common Log Messages

**FastComments Logs**:
- "SAML config not found" - SAML not enabled or misconfigured
- "Invalid certificate" - Certificate validation failed
- "Missing email attribute" - Required email not provided in SAML response

**IdP Logs**:
- "Unknown service provider" - Entity ID mismatch
- "Invalid ACS URL" - Assertion Consumer Service URL incorrect
- "User not assigned" - User lacks access to SAML application

### Getting Help

#### Information to Gather

When contacting support, provide:
- Exact error messages and timestamps
- SAML configuration details (without sensitive data)
- IdP type and version
- Steps to reproduce the issue
- Browser and network information

#### FastComments Support

For SAML-related issues:
1. Use the [portal de soporte](https://fastcomments.com/auth/my-account/help)
2. Include tenant ID and affected user emails
3. Provide error messages and configuration details
4. Specify IdP type and configuration approach

#### IdP Support

For IdP-specific issues:
- Consult IdP documentation for SAML troubleshooting
- Use IdP support channels for configuration problems
- Leverage IdP community forums for common issues

### Prevention Tips

#### Best Practices

1. **Test Thoroughly**:
   - Test configuration changes in non-production environment
   - Verify with multiple test users
   - Document working configurations

2. **Monitor Regularly**:
   - Set up monitoring for SAML authentication failures
   - Review certificate expiration dates
   - Monitor for IdP configuration changes

3. **Documentation**:
   - Maintain documentation of SAML configuration
   - Document any custom configurations or workarounds
   - Keep contact information for IdP administrators

#### Proactive Maintenance

1. **Certificate Management**:
   - Monitor certificate expiration dates
   - Plan certificate rotation procedures
   - Test certificate updates before expiration

2. **Configuration Reviews**:
   - Regularly review SAML configuration
   - Verify IdP configuration remains current
   - Update documentation as changes are made
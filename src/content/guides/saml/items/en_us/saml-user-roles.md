FastComments maps SAML user roles to internal permissions, enabling role-based access control for your organization.

### FastComments Role System

FastComments uses a role-based permission system where users can have one or more roles that determine their access levels and capabilities.

### Available FastComments Roles

#### Administrative Roles

**fc-account-owner**
- **Permissions**: Complete administrative access
- **Capabilities**: All features, billing management, user management
- **Use Case**: Primary account administrators and owners

**fc-admin-admin**  
- **Permissions**: Administrative access to most features
- **Capabilities**: User management, configuration, moderation. **Can administer other admins.**
- **Use Case**: Secondary administrators and IT staff

**fc-billing-admin**
- **Permissions**: Billing and subscription management
- **Capabilities**: Payment methods, invoices, subscription changes
- **Use Case**: Finance team members and billing contacts

#### Specialized Roles

**fc-analytics-admin**
- **Permissions**: Access to analytics and reporting
- **Capabilities**: View site statistics, user engagement data
- **Use Case**: Marketing teams and data analysts

**fc-api-admin**
- **Permissions**: API access and management
- **Capabilities**: API credentials, webhook configuration
- **Use Case**: Developers and technical integrators

**fc-moderator**
- **Permissions**: Comment moderation capabilities
- **Capabilities**: Approve/reject comments, manage spam
- **Use Case**: Community moderators and content managers

### Role Mapping Configuration

#### SAML Attribute Sources

FastComments accepts role information from various SAML attribute names to ensure compatibility with different identity providers:

**Standard Attribute Names**:
- `roles`
- `groups` 
- `memberOf`
- `role`
- `group`

**Microsoft/ADFS Attributes**:
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

#### Role Format Support

**Array Format** *(Preferred)*:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
    <saml:AttributeValue>fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Comma-Separated Format**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin,fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Single Role Format**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
</saml:Attribute>
```

### Identity Provider Role Configuration

#### Microsoft Azure AD

1. **App Roles Configuration**:
   - Define FastComments roles in your Azure AD application
   - Assign users to appropriate app roles
   - Configure claims to include assigned roles

2. **Attribute Mapping**:
   ```
   Attribute Name: roles
   Source Attribute: user.assignedroles
   ```

#### Okta

1. **Group Assignment**:
   - Create groups matching FastComments role names
   - Assign users to appropriate groups
   - Configure attribute statements

2. **Attribute Statement**:
   ```
   Name: roles
   Value: user.groups
   Filter: Starts with "fc-"
   ```

#### Google Workspace

1. **Group Mapping**:
   - Create organizational units or groups
   - Name groups with FastComments role prefixes
   - Configure attribute mapping

2. **Custom Attributes**:
   ```
   Attribute Name: roles
   Value: Groups or custom schema attribute
   ```

### Default User Behavior

#### Users Without Roles

When a SAML user has no roles or unrecognized roles:
- User is created as a standard commenter
- No administrative access is granted
- Can post and manage their own comments
- Cannot access admin dashboard features

#### Role Inheritance

- Users can have multiple roles simultaneously
- Permissions are cumulative (highest permission level applies)
- Role changes in IdP are reflected on next login

### Managing SAML Users

#### User Creation

When a user logs in via SAML for the first time:
1. **User Account**: Automatically created with email as identifier
2. **Role Assignment**: Roles applied based on SAML attributes
3. **Profile Information**: First/last name populated if provided
4. **Permission Activation**: Roles become active immediately

#### Role Updates

Existing SAML users receive role updates:
1. **Login Trigger**: Role updates occur during each SAML login
2. **Immediate Effect**: New permissions apply immediately
3. **Role Removal**: Removed roles are revoked automatically
4. **Audit Trail**: Role changes are logged in audit logs

### Custom Role Mapping

#### Enterprise Customization

For enterprise customers with specific requirements:
- Custom role names can be mapped to FastComments permissions
- Complex role hierarchies can be implemented
- Department-specific access controls can be configured

Contact FastComments support for custom role mapping configurations.

#### Role Validation

FastComments validates incoming roles:
- Unrecognized roles are ignored (not rejected)
- Malformed role attributes are logged for troubleshooting
- Users maintain existing roles if SAML assertion lacks role information

### Best Practices

#### Role Management

1. **Principle of Least Privilege**: Assign minimal necessary permissions
2. **Regular Auditing**: Review user roles and access periodically  
3. **Clear Naming**: Use descriptive group names in your IdP
4. **Documentation**: Maintain documentation of role assignments

#### Security Considerations

1. **Role Attributes**: Ensure role attributes are properly secured in SAML responses
2. **Attribute Validation**: Verify that only authorized systems can assign roles
3. **Access Reviews**: Regularly review administrative role assignments
4. **Monitoring**: Monitor role changes and administrative actions

### Troubleshooting Role Issues

#### Common Problems

**Roles Not Applied**:
- Check SAML attribute names match supported formats
- Verify IdP is sending role information
- Confirm role values match FastComments role names exactly

**Access Denied**:
- Verify user has appropriate role assigned in IdP
- Check role spelling and case sensitivity
- Confirm role is properly formatted in SAML response

**Missing Permissions**:
- Review role definitions and required permissions
- Check for conflicting role assignments
- Verify user has logged in after role changes

---
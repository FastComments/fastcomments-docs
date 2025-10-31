### Privacy Controls Overview

FastComments provides several privacy controls to help you comply with GDPR requirements for user consent, cookie management, and data processing transparency.

### Cookie Management

#### Understanding FastComments Cookies

FastComments uses cookies for:

- **Authentication** - Keeping users logged in
- **Preferences** - Remembering user settings
- **Functionality** - Enabling core commenting features

#### Cookie Consent

Under GDPR, you must:

1. **Obtain consent** - Get user permission before setting non-essential cookies
2. **Provide information** - Explain what cookies are used and why
3. **Allow opt-out** - Users must be able to refuse non-essential cookies

#### Implementing Cookie Consent

**Option 1: Use a Consent Management Platform (CMP)**

Integrate FastComments with your existing consent management solution:

1. Configure your CMP to recognize FastComments cookies
2. Only load FastComments after user grants consent
3. Conditionally initialize the widget based on consent status

Example:
```javascript
// Wait for consent before initializing
if (userHasGrantedConsent()) {
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: 'your-tenant-id'
    });
}
```

**Option 2: Built-in Privacy Settings**

Configure FastComments to minimize data collection:

- Disable analytics cookies if not needed
- Use session-only cookies where possible
- Implement cookie-less authentication options for anonymous comments

#### Third-Party Cookies

FastComments can operate without third-party cookies. See our [Third-Party Cookies Guide](/guide-third-party-cookies.html) for detailed information on:

- How FastComments handles third-party cookie restrictions
- Configuration options for cookie-less operation
- Impact on functionality and user experience

### Anonymous Commenting

#### Reducing Personal Data Collection

You can configure FastComments to allow anonymous commenting, which reduces the amount of personal data collected:

**Benefits:**
- Users can comment without creating an account
- Reduces personal data storage requirements
- Simplifies GDPR compliance for casual commenters

**Configuration:**
```javascript
FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: 'your-tenant-id',
    enableAnonymousComments: true
});
```

**Considerations:**
- Anonymous comments may increase moderation needs
- Limited ability to manage user reputation
- Users still have IP addresses logged for spam prevention

### Third-Party Integrations

#### Data Sharing and Processors

When using third-party integrations with FastComments, consider GDPR requirements:

**Social Login Integration:**
- Google, Facebook, Twitter authentication
- These services act as data processors
- Ensure you have appropriate Data Processing Agreements (DPAs)
- Inform users in your privacy policy

**Analytics Integration:**
- If you integrate FastComments with analytics tools
- Ensure analytics providers are GDPR-compliant
- Consider user consent requirements
- Use anonymization features where available

#### Managing Third-Party Data Access

**Best Practices:**
1. **Minimize integrations** - Only enable integrations you actually need
2. **Review DPAs** - Ensure third-party vendors have appropriate data processing agreements
3. **Update privacy policy** - Document all third-party data sharing
4. **Enable user controls** - Allow users to opt out of specific integrations

### Data Minimization

#### Collecting Only Necessary Data

GDPR requires that you collect only the data necessary for your specified purposes:

**Required for Commenting:**
- Comment text
- Timestamp
- User identifier (can be anonymous)

**Optional Data:**
- Display name
- Email address (for notifications)
- Profile picture
- Vote history

**Configuration Example:**
```javascript
FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: 'your-tenant-id',
    // Only collect essential data
    requireNameField: false,
    requireEmailField: false
});
```

### Consent Management

#### Obtaining Valid Consent

GDPR requires consent to be:

- **Freely given** - Users have a genuine choice
- **Specific** - Separate consent for different purposes
- **Informed** - Users understand what they're consenting to
- **Unambiguous** - Clear affirmative action required

#### Privacy Notices

Provide clear information to users:

1. **At point of data collection** - Inform users when they're about to comment
2. **In your privacy policy** - Detailed information about data processing
3. **Before enabling features** - Explain what features like notifications require

**Example Privacy Notice:**
```
By submitting a comment, you agree to our processing of your comment
text and optional profile information according to our Privacy Policy.
You can edit or delete your comments at any time.
```

### IP Address Handling

#### GDPR and IP Addresses

IP addresses are considered personal data under GDPR. FastComments collects IP addresses for:

- Spam prevention
- Security and abuse detection
- Geographic analytics (if enabled)

#### IP Address Options

**Anonymization:**
- Configure IP anonymization to remove the last octet
- Reduces precision while maintaining spam protection
- Balances functionality with privacy

**Retention:**
- Set IP address retention policies
- Delete old IP addresses after a defined period
- Document retention periods in your privacy policy

### Notification Controls

#### User Preferences

Users can control their notification preferences:

- **Email notifications** - Opt in/out of email alerts
- **Frequency** - Choose immediate, digest, or no notifications
- **Granular control** - Select which events trigger notifications

#### Consent for Marketing

If you send marketing emails through FastComments:

1. **Obtain explicit consent** - Separate from comment functionality consent
2. **Provide unsubscribe** - Easy opt-out in every email
3. **Honor requests promptly** - Process unsubscribe requests immediately

### Data Processing Transparency

#### Informing Users

Be transparent about how FastComments processes data:

1. **Privacy Policy** - Link to your privacy policy prominently
2. **Purpose limitation** - Clearly state why you collect each type of data
3. **Data retention** - Explain how long data is kept
4. **User rights** - Inform users of their GDPR rights

**Example:**
```javascript
FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: 'your-tenant-id',
    // Display privacy notice
    showPrivacyNotice: true,
    privacyPolicyUrl: 'https://yoursite.com/privacy'
});
```

### Compliance Checklist

Use this checklist to ensure your privacy controls are configured correctly:

- [ ] Cookie consent mechanism implemented
- [ ] Privacy policy updated with FastComments data processing details
- [ ] Data minimization settings configured
- [ ] Third-party integrations reviewed and documented
- [ ] User notification preferences enabled
- [ ] IP address handling policy established
- [ ] Anonymous commenting options considered
- [ ] Consent management integrated
- [ ] Privacy notices displayed at data collection points
- [ ] Data Processing Agreement with FastComments in place

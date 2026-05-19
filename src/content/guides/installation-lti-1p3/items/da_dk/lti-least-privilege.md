The FastComments LTI 1.3-integration følger princippet om mindst privilegium: den bruger kun de launch-claims, der er nødvendige for at identificere brugeren, knytte kommentarer til det korrekte kursus og den korrekte ressource samt anvende rollebaserede tilladelser.

The rest of this page maps every claim the integration consumes, every LTI Advantage service it does not request, and every category of data it does not collect. Security and procurement reviewers can lift answers directly from the tables below.

## Data Elements Received From the LMS

Every LTI 1.3 launch carries a signed JWT from the LMS. FastComments extracts the following claims from that JWT and uses nothing else:

| Field | LTI claim | Purpose | Required | Stored |
|-------|-----------|---------|----------|--------|
| User identifier | `sub` | Identifies the user consistently across launches so the same person resolves to the same FastComments SSO user | Yes | Yes, as part of a stable internal SSO ID |
| Display name | `name` | Attribution shown next to the user's comments | Yes (falls back to "LMS User" if absent) | Yes |
| Email | `email` | Account matching, notifications, moderation, support correspondence | Optional (the integration works without it) | Yes when provided |
| Avatar URL | `picture` | Displayed on the user's comments | Optional | URL only; FastComments does not download or rehost the image |
| Roles | `https://purl.imsglobal.org/spec/lti/claim/roles` | Determines whether the user is administrator, instructor (moderator), or learner | Yes | Derived `isAdmin` / `isModerator` flags on the SSO session |
| Course context | `https://purl.imsglobal.org/spec/lti/claim/context` (`id`, `title`) | Associates the comment thread with the correct LMS course | Yes | Yes, as part of the resolved page identifier |
| Resource link | `https://purl.imsglobal.org/spec/lti/claim/resource_link` (`id`) | Associates comments with the correct activity or tool placement inside the course | Yes when present | Yes, as part of the resolved page identifier |
| Deployment ID | `https://purl.imsglobal.org/spec/lti/claim/deployment_id` | Routes the launch to the correct FastComments tenant configuration | Yes | Yes, on the FastComments LTI configuration record |

## Claims and Scopes Declared at Registration

During LTI 1.3 Dynamic Registration, FastComments registers itself with `scope: ""` (no additional OAuth scopes) and declares only these OpenID Connect claims:

`iss`, `sub`, `name`, `email`, `picture`

It registers two message types:

- `LtiResourceLinkRequest` - the standard course launch into FastComments.
- `LtiDeepLinkingRequest` - allows instructors to place the FastComments tool inside a course.

No additional access tokens are requested from the LMS.

## LTI Advantage Services Not Requested

| Service / scope | Requested? | Reason |
|------------------|------------|--------|
| Names and Role Provisioning Services (NRPS) | No | The integration does not need a course roster; user identity arrives with each launch |
| Assignment and Grade Services (AGS) - lineitem, score, result scopes | No | The integration is not gradebook-aware |
| Deep Linking beyond the standard placement return | No additional data | Deep linking is used only for instructor placement of the tool; no course content is enumerated |

## Data Not Collected

Beyond LTI itself, FastComments does not request or receive the following from the LMS or user:

| Category | Collected? |
|----------|------------|
| Student grades | No |
| Assignment submissions | No |
| Attendance records | No |
| Full course rosters | No |
| Government identifiers | No |
| Date of birth | No |
| Postal address or phone number | No |
| Financial information | No |
| LMS administrator credentials | No |

## Access Boundaries

- FastComments only receives data inside an authorized LTI 1.3 launch signed by the LMS's registered keys. The integration does not call back into the LMS for additional information.
- Launch tokens are single-use and short-lived. Replayed or expired tokens are rejected.
- LMS administrators control where the tool is deployed inside their platform. D2L Brightspace, for example, supports per-deployment org-unit scoping and per-deployment security settings, which allows administrators to restrict the tool to specific courses or org units rather than making it available globally. Moodle, Blackboard, Sakai, and Schoology offer equivalent per-deployment controls in their LTI 1.3 implementations.

## Storage and Retention

FastComments retains LTI-derived data for the duration of the active commenting service and according to customer-configured retention settings. Comment data is stored in encrypted-at-rest production storage. On account termination or written deletion request, FastComments deletes or anonymizes customer data per the applicable agreement.

For full storage and data-handling details, see the <a href="https://fastcomments.com/privacy-policy" target="_blank">FastComments' privatlivspolitik</a>.

## Review Cadence

Any new LTI feature that would require additional claims, scopes, or LTI Advantage services is reviewed before release to confirm the requested access is necessary and proportionate to the feature being shipped.

## Short Statement for Security Questionnaires

> FastComments applies least privilege and data minimization to its LTI 1.3 integration. The integration uses only the LTI launch claims required to authenticate the user (`sub`, `name`, `email`, `picture`), determine their role, and identify the course and resource that comments belong to. FastComments does not request Names and Role Provisioning Services, Assignment and Grade Services, gradebook data, attendance, full rosters, or LMS administrative access. LMS administrators retain control over which org units, courses, and deployments the tool is available in.
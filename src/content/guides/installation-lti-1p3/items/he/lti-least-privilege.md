The FastComments LTI 1.3 integration follows the principle of least privilege: it uses only the launch claims required to identify the user, attach comments to the correct course and resource, and apply role-based permissions.

The rest of this page maps every claim the integration consumes, every LTI Advantage service it does not request, and every category of data it does not collect. Security and procurement reviewers can lift answers directly from the tables below.

## Data Elements Received From the LMS

Every LTI 1.3 launch carries a signed JWT from the LMS. FastComments extracts the following claims from that JWT and uses nothing else:

| Field | LTI claim | Purpose | Required | Stored |
|-------|-----------|---------|----------|--------|
| User identifier | `sub` | Identifies the user consistently across launches so the same person resolves to the same FastComments SSO user | כן | כן, כחלק ממזהה SSO פנימי יציב |
| Display name | `name` | Attribution shown next to the user's comments | כן (נכשל חזרה ל-"LMS User" אם חסר) | כן |
| Email | `email` | Account matching, notifications, moderation, support correspondence | אופציונלי (האינטגרציה עובדת בלעדיו) | כן כאשר נמסר |
| Avatar URL | `picture` | Displayed on the user's comments | אופציונלי | רק כתובת URL; FastComments לא מורידה או מארחת מחדש את התמונה |
| Roles | `https://purl.imsglobal.org/spec/lti/claim/roles` | Determines whether the user is administrator, instructor (moderator), or learner | כן | דגלי `isAdmin` / `isModerator` נגזרים על מושב ה-SSO |
| Course context | `https://purl.imsglobal.org/spec/lti/claim/context` (`id`, `title`) | Associates the comment thread with the correct LMS course | כן | כן, כחלק מזהה העמוד שנפתר |
| Resource link | `https://purl.imsglobal.org/spec/lti/claim/resource_link` (`id`) | Associates comments with the correct activity or tool placement inside the course | כן כאשר קיים | כן, כחלק מזהה העמוד שנפתר |
| Deployment ID | `https://purl.imsglobal.org/spec/lti/claim/deployment_id` | Routes the launch to the correct FastComments tenant configuration | כן | כן, ברשומת תצורת LTI של FastComments |

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
| Names and Role Provisioning Services (NRPS) | לא | האינטגרציה אינה זקוקה לרשימת תלמידים; זהות המשתמש מגיעה עם כל השקה |
| Assignment and Grade Services (AGS) - lineitem, score, result scopes | לא | האינטגרציה אינה מודעת ליומן ציונים |
| Deep Linking beyond the standard placement return | לא נדרשים נתונים נוספים | Deep linking משמשת רק למיקום הכלי על ידי המדריך; אין מיפוי של תוכן קורס |

## Data Not Collected

Beyond LTI itself, FastComments does not request or receive the following from the LMS or user:

| Category | Collected? |
|----------|------------|
| Student grades | לא |
| Assignment submissions | לא |
| Attendance records | לא |
| Full course rosters | לא |
| Government identifiers | לא |
| Date of birth | לא |
| Postal address or phone number | לא |
| Financial information | לא |
| LMS administrator credentials | לא |

## Access Boundaries

- FastComments only receives data inside an authorized LTI 1.3 launch signed by the LMS's registered keys. The integration does not call back into the LMS for additional information.
- Launch tokens are single-use and short-lived. Replayed or expired tokens are rejected.
- LMS administrators control where the tool is deployed inside their platform. D2L Brightspace, for example, supports per-deployment org-unit scoping and per-deployment security settings, which allows administrators to restrict the tool to specific courses or org units rather than making it available globally. Moodle, Blackboard, Sakai, and Schoology offer equivalent per-deployment controls in their LTI 1.3 implementations.

## Storage and Retention

FastComments retains LTI-derived data for the duration of the active commenting service and according to customer-configured retention settings. Comment data is stored in encrypted-at-rest production storage. On account termination or written deletion request, FastComments deletes or anonymizes customer data per the applicable agreement.

For full storage and data-handling details, see the <a href="https://fastcomments.com/privacy-policy" target="_blank">מדיניות הפרטיות של FastComments</a>.

## Review Cadence

Any new LTI feature that would require additional claims, scopes, or LTI Advantage services is reviewed before release to confirm the requested access is necessary and proportionate to the feature being shipped.

## Short Statement for Security Questionnaires

> FastComments applies least privilege and data minimization to its LTI 1.3 integration. The integration uses only the LTI launch claims required to authenticate the user (`sub`, `name`, `email`, `picture`), determine their role, and identify the course and resource that comments belong to. FastComments does not request Names and Role Provisioning Services, Assignment and Grade Services, gradebook data, attendance, full rosters, or LMS administrative access. LMS administrators retain control over which org units, courses, and deployments the tool is available in.
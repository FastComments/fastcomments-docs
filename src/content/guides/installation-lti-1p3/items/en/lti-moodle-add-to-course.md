This guide covers adding FastComments to a Moodle 4.x course after a site administrator has registered the tool and set it to show in the activity chooser. If FastComments is not yet registered, see the Moodle registration guide first.

#### Open the Course in Edit Mode

1. Sign in to Moodle as an Editing Teacher (or higher) for the course.
2. Open the course.
3. Toggle **Edit mode** on using the switch in the top-right corner of the course header.

Moodle 4.x replaced the legacy "Add an activity or resource" dropdown that 3.x used with a full-screen activity chooser dialog. Moodle 4.5 keeps the same chooser but adds a starred/favorites row at the top, so pinning FastComments once makes it faster to reach in later sections.

#### Add the FastComments Activity

1. Scroll to the course section (topic or week) where the discussion belongs.
2. Click **Add an activity or resource** at the bottom of that section.
3. In the chooser dialog, select **FastComments**. If you don't see it, jump to the gotchas section below.

The activity settings form opens. The fields that matter:

- **Activity name** (required). Shown on the course page and in the gradebook. Example: `Week 3 Discussion`.
- **Activity description**. Optional intro text rendered above the comment thread.
- **Show description on course page**. Tick this if you want the description visible without clicking into the activity.
- **Preconfigured tool**. Set to `FastComments` (auto-selected when launched from the chooser). Do not change.
- **Launch container**. Set to **New window**. See the gotchas section for why "Same window" breaks in some Moodle deployments.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. Leave blank. Dynamic Registration handled these at the site level.

Scroll to the bottom and click **Save and return to course** (or **Save and display** to open the activity right away).

The activity appears as a row in the section with the FastComments icon. Students click the row to open the comment thread.

#### Embed FastComments Inline with the Editor

For a thread inside a Page, Book chapter, Lesson, or any other resource that uses the Atto or TinyMCE editor:

1. Open the resource in edit mode.
2. Place the cursor where the thread should appear.
3. In the editor toolbar, click the **LTI** / **External tool** button. In Atto it's labeled "Insert LTI Advantage content". In TinyMCE (default in Moodle 4.3+) it's under the **More** menu as **External tools**.
4. Pick **FastComments** from the tool list.
5. FastComments opens a deep-linking picker. Confirm the thread title and click **Embed**.
6. The editor inserts an LTI placeholder block. Save the resource.

Each embedded instance is a distinct thread keyed on the deep-link content item ID, so a Page with three FastComments embeds gets three independent threads.

#### Restrict Access and Group Settings

The standard Moodle activity settings apply to FastComments activities:

- **Common module settings** > **Group mode**. Setting this to **Separate groups** or **Visible groups** does not split FastComments into per-group threads on its own. Moodle's group mode only filters the gradebook and member list. To run a separate thread per group, add one FastComments activity per group and use **Restrict access** to scope each one.
- **Restrict access** > **Add restriction**. Supports the standard Moodle conditions: **Date**, **Grade**, **Group**, **Grouping**, **User profile**, and nested restriction sets. Use **Group** to lock a FastComments activity to a single group.
- **Activity completion**. Set to **Students must view this activity to complete it** if you want completion tracking. FastComments does not currently report a completion event back to Moodle beyond the launch.

#### Role Mapping

FastComments reads the LTI `roles` claim that Moodle sends on every launch and maps it as follows:

- Moodle **Manager** or **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** or **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> read-only

Admins can delete any comment, ban users, and edit thread settings. Moderators can delete and approve comments inside the thread they launched into. Custom Moodle roles inherit the mapping of the archetype they were cloned from.

#### What Students See

Students click the FastComments activity (or scroll to the embedded block inside a Page or Book). Moodle sends their identity to FastComments via the LTI launch:

- No login screen. FastComments signs them in using the Moodle account.
- Their display name, email, and avatar come from Moodle.
- The thread is scoped to `(Moodle site, course, resource link ID)`, so the same activity duplicated into another course gets a fresh thread.
- Threaded replies, voting, and notifications work the same as a standalone FastComments thread.

#### Lock Down Public Access (Recommended)

By default, FastComments comment data is publicly readable. Anyone who can guess a thread's URL or API endpoint can view its comments, even outside Moodle. For course discussions you almost certainly want to restrict viewing to enrolled students only.

Open your <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget customization page</a> and create a rule with **Require SSO To View Comments** enabled, then set the security level to **Secure SSO** so threads can only be loaded through the signed LTI launch.

See [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) for the full walkthrough, including how to scope the rule to a single domain or page.

#### Moodle Gotchas

**FastComments missing from the activity chooser.** The site administrator registered the tool but didn't set **Tool configuration usage** to **Show in activity chooser and as a preconfigured tool**. Fix this under **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > gear icon on the FastComments tile.

**Launch fails or shows a blank frame when set to "Same window".** Moodle's session cookies use `SameSite=Lax` by default, and some browsers strip them on the cross-site POST that LTI 1.3 uses to return from FastComments. Set **Launch container** to **New window** on the activity. This is a hard requirement for embedded FastComments inside a Page or Book, since the editor-embedded launch path always pops a new window.

**The `iss` claim is the Moodle site URL, not a tenant ID.** FastComments uses the Moodle site URL (the `wwwroot` config value) as the LTI issuer. If your Moodle instance moves to a new domain or you change `wwwroot`, existing FastComments threads stay tied to the old issuer and won't match new launches. Re-register the tool against the new URL and migrate threads through the FastComments admin if needed.

**Activity backup and restore.** Backing up a course and restoring it into a new course creates new resource link IDs, so the restored FastComments activities start with empty threads. The original course retains the original threads. This is intended behavior, not a bug.

**Moodle 4.5 TinyMCE default.** Moodle 4.5 ships with TinyMCE as the default editor for new installs. The External tool button location is under the **More** (`...`) menu rather than the main toolbar. Older sites that upgraded from 4.1 keep Atto unless an admin switched the default.

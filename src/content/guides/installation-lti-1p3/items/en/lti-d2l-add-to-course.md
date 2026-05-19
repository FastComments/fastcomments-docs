This page covers adding FastComments to a Brightspace course after an administrator has registered the tool and created a deployment. If the tool is not registered yet, see the D2L registration guide first.

<div class="screenshot white-bg">
    <div class="title">FastComments embedded as a unit topic in Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments running inside a Brightspace unit, showing threaded comments and an @-mention picker" />
</div>

Brightspace ships two content authoring experiences: **Classic Content** and the **New Content Experience** (also called **Lessons**). Both expose FastComments, but the menu paths differ. Each section below covers both where they diverge.

#### Locate the FastComments Tool

The FastComments tool appears in two places inside a course content editor:

1. The activity picker, reached from a module/unit's **Add Existing** button (labeled **Add Existing Activities** in older Brightspace versions). FastComments shows up directly in the picker in current Brightspace builds; older versions nest it under an **External Learning Tools** submenu. Either path adds FastComments as a standalone topic.
2. The **Insert Stuff** dialog inside the HTML editor, under **LTI Advantage**. This embeds FastComments inline in an HTML topic via the LTI deep linking flow.

If FastComments does not appear in either picker, the deployment is not enabled for the org unit holding the course. Ask your Brightspace administrator to open **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, open the deployment, and add the course's org unit (or a parent org unit) under **Org Units**.

#### Add FastComments as a Topic in a Module

Classic Content:

1. Open the course and click **Content** in the navbar.
2. Select the module that should hold the discussion (or create one via **Add a module**).
3. Click **Add Existing** (older Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. In the picker, click **FastComments**. Brightspace creates a topic in the module and returns you to the content view.
5. Click the new topic. Rename it to something descriptive like `FastComments Discussion` using the inline title editor.

New Content Experience (Lessons):

1. Open the course and click **Content**.
2. Open the unit and lesson that should hold the discussion.
3. Click **Add** > **Existing Activity** and select **FastComments** (older Brightspace: nested under **External Learning Tools**).
4. The activity is added to the lesson.
5. Click the activity title to rename it.

The first time any user (instructor or student) opens the topic, FastComments initializes the thread for that resource link. The thread is bound to the resource link ID, so renaming or moving the topic does not change which thread is loaded.

#### Embed FastComments Inline in an HTML Topic

Use this flow when you want comments to appear below a reading, video, or other content inside the same topic page rather than as a separate topic.

1. Open or create an HTML topic in the module/lesson.
2. Click **Edit HTML** to open the Brightspace HTML editor.
3. Place the cursor where the comment thread should appear.
4. Click the **Insert Stuff** button (puzzle-piece icon in the editor toolbar).
5. In the Insert Stuff dialog, scroll to **LTI Advantage** and click **FastComments**.
6. FastComments opens a deep linking picker. Confirm the placement (the default options work for content discussions); click **Insert** or **Continue**.
7. Brightspace returns to the HTML editor with a placeholder block representing the LTI launch. Click **Save and Close** on the topic.

When the topic loads, Brightspace replaces the placeholder with an iframe that auto-launches FastComments via LTI. Students see the discussion thread inline.

A single HTML topic holds multiple deep-linked FastComments embeds. Each embed gets its own thread because each deep link produces a distinct resource link ID.

#### Module Topic vs Inline Quicklink

Pick the **module topic** approach when:

- The discussion is the primary activity for that step in the module.
- You want the topic to appear in Brightspace's table of contents, completion tracking, and Class Progress.

Pick the **inline embed** approach when:

- Comments should sit below other content on the same page.
- You do not want a separate completion-trackable item in the table of contents.

#### Visibility, Draft, and Release Conditions

A new FastComments topic is visible to students by default. To hide it while you set it up:

1. In the content editor, click the topic title (Classic) or the three-dot menu on the activity (New Content Experience).
2. Set status to **Draft** (Classic) or toggle **Visibility** off (New Content Experience).

Draft topics are invisible to students. Instructors and TAs still see them with a "Draft" badge.

To restrict the topic to a specific group or section:

1. Open the topic.
2. Click the topic title menu > **Edit Properties In-place** (Classic) or **Edit** > **Restrictions** (New Content Experience).
3. Under **Release Conditions**, click **Create**.
4. Pick **Group enrollment** or **Section enrollment**, select the group/section, and save.

Release conditions stack with FastComments's own role mapping. Students who cannot see the topic do not get an LTI launch.

#### What Students See on First Launch

When a student clicks the topic (or loads an HTML topic with an embed):

1. Brightspace performs the LTI 1.3 launch in the background.
2. FastComments receives the student's name, email, avatar URL, and LMS role, and signs them in automatically. There is no FastComments login prompt.
3. The comment thread for that resource link renders inside the Brightspace iframe.

Role mapping at launch:

- Brightspace `Administrator` becomes a FastComments **admin** for the thread (full moderation, delete, ban, and configuration access).
- Brightspace `Instructor` becomes a FastComments **moderator** (pin, hide, delete, ban).
- All other roles (`Learner`, `TeachingAssistant`, etc.) become standard commenters.

Comments are attributed to the student's Brightspace account. If the student edits their name or avatar in Brightspace, the next LTI launch syncs the change.

#### Lock Down Public Access (Recommended)

By default, FastComments comment data is publicly readable. Anyone who can guess a thread's URL or API endpoint can view its comments, even outside Brightspace. For course discussions you almost certainly want to restrict viewing to enrolled learners only.

Open your <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget customization page</a> and create a rule with **Require SSO To View Comments** enabled, then set the security level to **Secure SSO** so threads can only be loaded through the signed LTI launch.

See [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) for the full walkthrough, including how to scope the rule to a single domain or page.

#### Iframe Height and Resize

FastComments emits the `org.imsglobal.lti.frameResize` postMessage on every thread render and on content changes (new comment, expand replies). Brightspace listens for this message and adjusts the iframe height so the thread is not clipped and does not show an inner scrollbar.

If the iframe stays at a fixed short height:

- Confirm the course is loaded over HTTPS. Brightspace's postMessage listener rejects mixed-content frames.
- Confirm no browser extension is blocking the postMessage channel.
- For inline embeds in an HTML topic, the surrounding HTML must not wrap the iframe in a fixed-height container. Remove any inline `style="height: ..."` from the parent element.

#### Brightspace-Specific Gotchas

**Tool not showing in the Add Existing picker.** The deployment is not enabled for this course's org unit. An administrator needs to add the org unit (or a parent) to the deployment's **Org Units** list. Tool registration alone is not enough; the deployment scopes which courses see the tool.

**`deployment_id` mismatch on launch.** FastComments TOFU-pins the first `deployment_id` it sees for a registration. If an administrator deletes the original deployment and creates a new one, launches from the new deployment are rejected with a deployment mismatch error. The fix is to re-register FastComments (generate a new registration URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">get it here</a>) and run Dynamic Registration again); the old configuration record is replaced.

**Tool launches but shows "Invalid LTI launch".** The course is in a different tenant/org structure than the deployment covers, or the deployment was disabled after registration. Re-check **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** toggle and the deployment's org unit list.

**Names and roles missing inside FastComments.** Brightspace ships LTI launches with Names and Role Provisioning Services (NRPS) claims. If a course was upgraded from an older LTI 1.1 link, the launch lacks `name` and `email` claims. Re-add the FastComments topic via **Add Existing** (do not migrate the old link) so the launch uses LTI 1.3.

**Embed shows a login screen instead of auto-SSO.** The HTML topic was inserted as a plain `<iframe>` pointing at FastComments rather than via **Insert Stuff** > **LTI Advantage**. Plain iframes skip the LTI launch and land users on the public-facing FastComments page. Delete the iframe and re-insert via the Insert Stuff flow.

Once FastComments is registered with the platform, instructors add it to course content using the platform's standard external tool flows. This page covers Sakai 23.x and Schoology Enterprise.

#### Sakai

**1. Add FastComments to a site**

The site maintainer enables the tool on a per-site basis:

1. Open the site and click **Site Info** in the left navigation.
2. Click **Manage Tools**.
3. Scroll to the **External Tools** list and toggle **FastComments** on.
4. Click **Continue**, review the tool list, then click **Finish**.

FastComments now appears as a left-nav item in the site.

**2. Reorder the left-nav entry**

Go to **Site Info** > **Tool Order**. Drag **FastComments** to the desired position and click **Save**. You can also rename the nav label and hide it from students from this screen.

**3. Embed inline in a Lessons page**

To place FastComments directly inside a Lessons page rather than as a standalone left-nav tool:

1. Open the **Lessons** tool in the site.
2. Click **Add Content** > **Add External Tool**.
3. Select **FastComments** from the list.
4. If FastComments advertised Deep Linking during registration, Sakai opens the tool's content selector so you can pick or label the thread. If Deep Linking wasn't advertised, Sakai inserts a default launch link.
5. Save the Lessons item.

Each embedded instance gets its own thread, scoped to that resource link.

**4. Permission tweaks for student access**

Sakai gates external tool launches through Realms. To confirm students can launch FastComments:

1. Sign in as a Sakai admin and open **Administration Workspace** > **Realms**.
2. Open the relevant realm (for example, `!site.template.course` or the specific site realm).
3. Confirm the `access` role has `lti.launch` enabled and that the role permissions in the **external.tools** group are granted.
4. Save the realm.

For site-level overrides, the maintainer can adjust per-role tool visibility from **Site Info** > **Tool Order** by hiding or showing FastComments per role.

**5. What students see**

Students click the FastComments left-nav item (or scroll to the embedded Lessons block) and land directly in the threaded comment view. SSO is automatic: Sakai sends the user's identity in the LTI launch and FastComments signs them in under their Sakai account.

Role mapping:

- Sakai `Instructor` -> FastComments moderator
- Sakai `Admin` (admin in Administration Workspace) -> FastComments admin
- Sakai `Student` / `access` -> FastComments commenter

**6. Sakai gotchas**

- **Tool not visible in Manage Tools.** If FastComments doesn't appear in the External Tools list, the Sakai admin needs to open the tool registry (**Administration Workspace** > **External Tools** > **FastComments**) and set **Stealthed** to `false`. Stealthed tools are hidden from the per-site Manage Tools picker.
- **Launches breaking in shared-session browsers.** Sakai's portal CSRF token is bound to the browser session. If a student is signed in to two Sakai sites in different tabs or has a stale session, the launch returns a 403. Fix: close other Sakai tabs, sign out, sign back in, and relaunch. Admins can also raise `sakai.csrf.token.cache.ttl` if this happens cluster-wide.
- **Frame embedding.** Confirm `lti.frameheight` in `sakai.properties` is large enough (600 or higher) so the comment thread isn't clipped inside a Lessons page.

#### Schoology

Schoology Enterprise has two installation scenarios. Confirm which one applies before adding the tool to a course.

**1. Two installation scenarios**

- **(a) Enterprise-level install.** The Schoology System Administrator installed FastComments at the organization level and assigned it to all courses or to specific course templates. Instructors skip installation and go straight to "Add Materials".
- **(b) Instructor self-install.** The instructor installs the tool into a single course from **Course Options** > **External Tools** > **Install LTI Apps**. Self-install requires the System Administrator to have approved the FastComments app at the org level first.

**2. Add FastComments as a course material**

Inside the course:

1. Open the course and go to **Materials**.
2. Click **Add Materials** > **Add File/Link/External Tool**.
3. Choose **External Tool**.
4. Select **FastComments** from the registered tools list.
5. Set a **Name** (this is what students see in the materials list) and an optional **Description**.
6. Leave **Enable Grading** (grade passback) **OFF**. FastComments does not report grades back to Schoology, so enabling grade passback creates an empty gradebook column.
7. Click **Submit**.

The material now appears in the course materials list and opens the FastComments thread when clicked.

**3. Inline embedding via the Rich Text editor**

If the System Administrator enabled Deep Linking placement for FastComments during registration, instructors can embed the comment thread inside any Rich Text field (assignment instructions, page bodies, discussion prompts):

1. Open the Rich Text editor on the target page.
2. Click the **External Tool** (puzzle piece) icon in the toolbar.
3. Choose **FastComments**.
4. Configure the embed in the deep-linking dialog and click **Insert**.
5. Save the page.

If the External Tool button doesn't appear in the Rich Text editor, Deep Linking is disabled for this tool on this tenant. See the gotchas below.

**4. Visibility and section assignments**

Schoology scopes tool availability per section through Course Options:

1. From the course, click **Course Options** > **External Tools**.
2. For each installed LTI app, you control whether it's available to all sections in the course or to specific sections.
3. To restrict FastComments to certain sections, uncheck the sections that should not see the tool.
4. Section-level access also gates which sections see the **Add Materials** > **External Tool** entry for FastComments.

**5. What students see**

Students click the FastComments material (or scroll to the inline embed) and land in the threaded discussion. SSO is automatic via the Schoology LTI launch under their Schoology account.

Role mapping:

- Schoology `Administrator` -> FastComments admin
- Schoology `Instructor` -> FastComments moderator
- Schoology `Student` -> FastComments commenter

**6. Schoology gotchas**

- **Enterprise-only.** Personal and free Schoology accounts cannot install LTI 1.3 tools. If your tenant is on the free tier, the **External Tools** option is absent from Course Options. Upgrade to Schoology Enterprise to use FastComments.
- **Deep Linking disabled by tenant default.** Some Schoology tenants restrict Deep Linking placement at the org level. When this is the case, instructors see only the **Add Materials** > **External Tool** flow and not the External Tool button in the Rich Text editor. To enable inline embedding, the System Administrator goes to **System Settings** > **Integration** > **LTI 1.3** > **FastComments** and enables the **Content Item / Deep Linking** placement, then saves.
- **Per-section assignment override.** If FastComments is assigned at the enterprise level but the instructor cannot see it in **Add Materials**, the course's section is excluded in the org-level assignment. Ask the System Administrator to add the section to the FastComments app assignment.
- **Material name vs. thread identity.** Renaming the material in Schoology does not move the comment thread. Threads are keyed on the LTI resource link ID, so a rename keeps the same thread; deleting and recreating the material creates a new, empty thread.
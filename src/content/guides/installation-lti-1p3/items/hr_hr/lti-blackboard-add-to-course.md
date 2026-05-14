Once an administrator has registered FastComments as an LTI 1.3 Advantage tool and approved the institution policies, instructors add it to courses through the standard Blackboard placement points. The exact steps differ between Ultra Course View and Original Course View, so both are covered below.

#### Ultra Course View

Ultra Course View is the default in Blackboard Learn SaaS as of 2026.

1. Open the course and go to the **Course Content** page.
2. Hover or tap where you want the comment thread to land in the outline and click the purple **+** (Add content) button.
3. Choose **Content Market**. The Content Market panel lists every approved LTI tool and Building Block placement for your institution.
4. Find the **FastComments** tile and click it. Blackboard creates a content item at the position where you opened the **+** menu.
5. The item lands in the outline as a "Visible to students" entry by default for instructors who have **Hide from students** off as their personal default. If your default is **Hidden**, the item is created hidden and you flip the visibility selector on the item row when you're ready.
6. To rename the item, click the title in the outline and type a new label. The title students see in the outline is independent of the FastComments thread identifier, so renaming is safe at any time.

If you don't see **Content Market** as an option, your institution has the placement hidden. You also reach the same picker through **More tools** in the same **+** menu under the **LTI Tools** group.

#### Original Course View

Original Course View is still supported in Learn SaaS and remains the primary experience for self-hosted Learn 9.1 sites on the Q4 2024 CU release line.

1. Open the course and enter a **Content Area** (for example, the default **Information** or **Content** area in the course menu).
2. Turn **Edit Mode** on with the toggle in the top-right of the page.
3. Click **Build Content** in the action bar.
4. Under the **Learning Tools** submenu, click **FastComments**. The Learning Tools submenu is populated from LTI 1.3 tool placements after an administrator registers the tool. If you don't see it, see the gotchas section below.
5. On the **Create FastComments** form, set:
   - **Name**: the label students see in the content area.
   - **Description**: optional text shown above the embedded thread.
   - **Permit Users to View this Content**: Yes/No availability toggle.
   - **Track Number of Views**: enable if you want Blackboard's per-item view statistics. FastComments runs its own analytics independently.
   - **Date and Time Restrictions**: optional **Display After** / **Display Until** windows.
6. Submit. The tool appears as a clickable item in the content area.

#### Embedding Inside an Item or Document

In both course views, instructors embed FastComments inline inside the body of an Item, Document, or any rich-text field through the Content Editor's LTI Advantage button.

Ultra Course View:

1. Create or edit a **Document**.
2. Click **Add content** inside the document body where you want the thread to appear.
3. In the editor toolbar, open the **Insert content** menu and click **Content Market** (the LTI Advantage / Deep Linking entry point).
4. Pick **FastComments**. FastComments returns a deep-link payload and Blackboard inserts an embedded block in the document body at the cursor position.
5. Save the document. Students see the thread rendered inline as they scroll past it.

Original Course View:

1. Edit any item with a rich-text body.
2. In the Content Editor toolbar, click the **Add Content** plus icon and choose **Content Market** (labeled **Add Content from External Tool** in older Q4 2024 CUs).
3. Pick **FastComments**. The editor inserts a placeholder block referencing the deep-linked resource.
4. Submit the item.

Each deep-link embed produces its own FastComments thread, so an Item with two embedded FastComments blocks has two independent comment streams.

#### Visibility, Release Conditions, and Group Restrictions

FastComments content items behave like any other Blackboard content item for the access control rules layered on top of them.

- Ultra: click the visibility selector on the row (**Visible to students**, **Hidden from students**, **Conditional availability**). Conditional availability supports date/time windows, performance rules against gradebook items, and member rules against course groups.
- Original: open the item's context menu and choose **Adaptive Release** or **Adaptive Release: Advanced** to gate the tool by date, membership, grade, or review status. Use **Set Group Availability** on the item to restrict to specific course groups.

FastComments respects whatever Blackboard's gate decides. If Blackboard hides the item from a student, the LTI launch never happens for that student and they don't appear in the moderator view.

#### Gradebook Behavior

FastComments does not report grades back over LTI Advantage Assignment and Grade Services. No grade column is auto-created for FastComments content items.

If your Blackboard tenant is configured to auto-create a gradebook column for every new content item regardless of grading metadata, an empty column appears anyway. To hide it:

- Ultra: open the **Gradebook**, click the column header, choose **Edit**, and turn off **Show to students** plus **Include in calculations**. Or use **Delete** if your institution permits column deletion for ungraded items.
- Original: open the **Grade Center**, click the column's chevron, choose **Hide from Users (on/off)**, and optionally **Hide from Instructor View** under **Column Organization**.

#### What Students See

When a student opens the FastComments item or scrolls to an embedded block:

1. Blackboard launches the LTI 1.3 message to FastComments. The student is signed in via SSO using their Blackboard identity (name, email, avatar, role) without seeing a login form.
2. The comment thread renders in the iframe. Threading, replies, mentions, and reactions are all available based on the comment widget settings configured in FastComments.
3. Their comments are attributed to their Blackboard account. If the student edits their name or photo in Blackboard later, the next launch updates the FastComments profile.

Role mapping from Blackboard to FastComments:

- **System Administrator** and **Course Builder** map to FastComments **admin**.
- **Instructor** and **Teaching Assistant** map to FastComments **moderator**.
- **Student**, **Guest**, and **Observer** map to FastComments **commenter**.

Moderators see moderation controls (pin, hide, ban, delete) inline on every comment in the thread.

#### Thread Scoping

FastComments scopes each thread by **(Blackboard host, course ID, resource link ID)**. Two FastComments items in the same course produce two threads. The same item copied across two course shells (for example, through course copy) produces two threads, because Blackboard issues a fresh resource link ID during the copy. To keep a shared thread across course copies, use Deep Linking with an explicit thread URN configured in FastComments before launching the copy.

#### Blackboard-Specific Gotchas

**FastComments tile missing from the Build Content menu (Original) or Content Market (Ultra).** The administrator approved the tool but left an institution policy blocking the relevant placement. Go to **Administrator Panel** > **Integrations** > **LTI Tool Providers**, edit the FastComments entry, and confirm both **Course Content Tool** (Original) and **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) placements are enabled. Save and refresh the course page.

**"Tool not configured for this context" or "Tool is not deployed" error on launch.** The deployment scope registered during dynamic registration doesn't match the institution context the course belongs to. In Blackboard's tool provider entry, verify the **Deployment ID** matches what FastComments shows on its LTI 1.3 Configuration page for this tenant. If they differ, delete the placement and re-run dynamic registration from a fresh registration URL.

**Iframe height looks fixed or content gets cut off.** Some Blackboard tenants ship with a strict Content Security Policy that blocks the default LTI iframe-resize postMessage. FastComments emits both the Canvas-style `lti.frameResize` message and the IMS spec-form `org.imsglobal.lti.frameResize` message to maximize compatibility, but a tenant-level CSP override blocks the parent listener. Ask your administrator to confirm that `*.fastcomments.com` is on the LTI tool allowlist and that no custom CSP header is stripping postMessage events. Resize then works without further configuration.

**Course copy duplicates threads.** Blackboard course copy issues new resource link IDs for LTI placements, so copied courses start with empty threads. This is expected. If you need the copied course to inherit the original thread, set up Deep Linking with an explicit thread URN before copying, or contact FastComments support to remap thread IDs in bulk.

**Student sees a generic Blackboard error on launch.** The cause is a missing or stale `email` claim. Confirm the institution policy for FastComments has **Role**, **Name**, and **Email Address** enabled under **User Fields to Send**. Save, then launch again in a fresh browser session.
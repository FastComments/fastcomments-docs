### Real-Time Updates

Collab Chat uses WebSocket connections to sync all conversations in real-time across all connected users. When someone creates a new annotation, adds a comment, or deletes a discussion, all other users viewing the same page see the update immediately without refreshing.

### How WebSocket Sync Works

When you initialize Collab Chat, the widget establishes a WebSocket connection to the FastComments servers. This connection remains open for the duration of the user's session and listens for updates related to the current page.

The WebSocket system uses three types of broadcast messages for Collab Chat. The `new-text-chat` event fires when someone creates a new annotation on the page. The `updated-text-chat` event fires when someone updates an existing conversation. The `deleted-text-chat` event fires when someone deletes an annotation.

### Broadcast ID System

To prevent echo effects where users see their own actions broadcast back to them, each update includes a unique `broadcastId`. When a user creates or updates an annotation, their client generates a UUID for that operation. When the WebSocket broadcasts the update back to all clients, the originating client ignores the update because it matches its own `broadcastId`.

This ensures smooth interaction where users see their changes immediately in the UI without waiting for the round-trip through the server, while still ensuring all other users get the update.

### Live User Count

The top bar displays the number of users currently viewing the page. This count updates in real-time as users join and leave. The user count is provided through the same WebSocket connection and increments/decrements automatically based on connection and disconnection events.

### Connection Resilience

If the WebSocket connection drops due to network issues or server maintenance, the widget automatically attempts to reconnect. During the reconnection period, users can still interact with existing annotations, but they won't see real-time updates from other users until the connection is reestablished.

Once reconnected, the widget resynchronizes to ensure no updates were missed. This happens transparently without requiring user intervention.

### Bandwidth Considerations

WebSocket messages are lightweight and contain only the essential information needed to sync state. Creating a new annotation typically uses less than 1KB of bandwidth. The system also includes intelligent batching to reduce message frequency during high-activity periods.

Your usage metrics in the FastComments dashboard track `pubSubMessageCount` and `pubSubBandwidth` so you can monitor real-time sync activity across your sites.

### Cross-Tab Synchronization

If a user has the same page open in multiple browser tabs, updates in one tab appear immediately in the other tabs. This works through the same WebSocket sync mechanism and doesn't require any additional configuration.

### Security

WebSocket messages are transmitted over secure connections (WSS) and include tenant validation to ensure users only receive updates for conversations they're authorized to see. The server validates all operations before broadcasting them to prevent unauthorized access or manipulation.

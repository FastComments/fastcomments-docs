### Real-Time Updates

Image Chat uses WebSocket connections to sync all conversations in real-time across all connected users. When someone creates a new marker, adds a comment, or deletes a discussion, all other users viewing the same image see the update immediately without refreshing.

### How WebSocket Sync Works

When you initialize Image Chat, the widget establishes a WebSocket connection to the FastComments servers. This connection remains open for the duration of the user's session and listens for updates related to the current image.

The WebSocket system uses three types of broadcast messages for Image Chat. The `new-image-chat` event fires when someone creates a new marker on the image. The `image-chat-updated` event fires when someone updates an existing conversation. The `deleted-image-chat` event fires when someone deletes a marker.

### Broadcast ID System

To prevent echo effects where users see their own actions broadcast back to them, each update includes a unique `broadcastId`. When a user creates or updates a marker, their client generates a UUID for that operation. When the WebSocket broadcasts the update back to all clients, the originating client ignores the update because it matches its own `broadcastId`.

This ensures smooth interaction where users see their changes immediately in the UI without waiting for the round-trip through the server, while still ensuring all other users get the update.

### Connection Resilience

If the WebSocket connection drops due to network issues or server maintenance, the widget automatically attempts to reconnect. During the reconnection period, users can still interact with existing markers, but they won't see real-time updates from other users until the connection is reestablished.

Once reconnected, the widget resynchronizes to ensure no updates were missed. This happens transparently without requiring user intervention.

### Bandwidth Considerations

WebSocket messages are lightweight and contain only the essential information needed to sync state. Creating a new marker typically uses less than 1KB of bandwidth. The system also includes intelligent batching to reduce message frequency during high-activity periods.

Your usage metrics in the FastComments dashboard track `pubSubMessageCount` and `pubSubBandwidth` so you can monitor real-time sync activity across your sites.

### Cross-Tab Synchronization

If a user has the same page open in multiple browser tabs, updates in one tab appear immediately in the other tabs. This works through the same WebSocket sync mechanism and doesn't require any additional configuration.

Users can have your site open on multiple devices simultaneously, and all of them will stay in sync. A marker created on a desktop computer appears instantly on the user's tablet if both devices are viewing the same image.

### Security

WebSocket messages are transmitted over secure connections (WSS) and include tenant validation to ensure users only receive updates for conversations they're authorized to see. The server validates all operations before broadcasting them to prevent unauthorized access or manipulation.

### Offline Behavior

When users are completely offline, they can still view existing markers but cannot create new ones or see updates from others. The widget detects the offline state and displays appropriate messaging.

If a user attempts to create a marker while offline and then comes back online, the operation will fail rather than queue, ensuring data consistency. Users should retry the operation once their connection is restored.

### Performance Impact

The WebSocket connection has minimal performance impact. The connection remains idle when no updates are occurring and only processes messages when activity happens. On a typical image with moderate marker activity, the WebSocket uses less CPU than rendering the image itself.

For pages with hundreds of simultaneous users and high marker creation activity, the system scales horizontally to maintain performance without impacting individual client connections.

### Collaborative Use Cases

The real-time sync makes Image Chat particularly powerful for collaborative workflows. Design teams can review mockups together with everyone seeing marker placements in real-time. Customer support teams can collaboratively annotate screenshots to identify issues. Educational groups can discuss diagrams with all participants seeing each other's markers as they're created.

The immediate feedback creates a more engaging and productive collaborative experience compared to traditional comment systems where users need to refresh to see updates.
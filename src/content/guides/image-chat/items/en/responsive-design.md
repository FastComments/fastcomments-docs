### Percentage-Based Positioning

Image Chat uses percentage-based coordinates instead of pixel coordinates to position chat markers on images. When a user clicks on an image, the widget converts the pixel coordinates of the click into percentages of the image width and height. This approach ensures that markers remain in the correct location regardless of how the image is displayed.

For example, if a user clicks 250 pixels from the left edge of a 1000px wide image, the widget stores this as 25% from the left. When another user views the same image at 500px wide on a mobile device, the marker appears at 125 pixels from the left, which is still 25% of the width.

### Benefits for Responsive Layouts

This percentage system makes Image Chat work seamlessly across all device sizes and orientations. Your images might be displayed at different sizes depending on screen width, CSS rules, or container constraints, but the markers always align correctly with the intended locations.

Users on desktop computers with large monitors see markers in the same relative positions as users on smartphones with small screens. The markers scale proportionally with the image itself.

### Image Scaling and Aspect Ratio

As long as your image maintains its aspect ratio when scaling (which is the default browser behavior), the percentage-based markers will remain perfectly aligned. The widget assumes that images scale proportionally and calculates positions based on this assumption.

If you apply CSS that distorts the image aspect ratio (like using `object-fit: cover` with specific dimensions), the marker positions may not align correctly. For best results, allow images to scale naturally or use `object-fit: contain` to maintain aspect ratio.

### Chat Square Sizing

The visual size of chat markers is also percentage-based. The `chatSquarePercentage` configuration option defaults to 5%, meaning each square is 5% of the image width. This ensures consistent visual weight across different image sizes.

On a 1000px wide image with the default 5% setting, markers are 50px square. On a 500px wide image, the same markers are 25px square. They remain proportional to the image size.

### Mobile Behavior

On screens under 768px wide, Image Chat switches to a mobile-optimized layout. Chat windows open fullscreen instead of floating next to the marker. This provides better usability on small screens where floating windows would obscure too much of the image.

The markers themselves remain visible and clickable at their percentage-based positions. Users can still see where all discussions are located and tap markers to open the fullscreen chat interface.

### Dynamic Image Loading

The percentage-based system works correctly even when images load asynchronously or change size after the page loads. The widget monitors the image element and recalculates marker positions whenever the image dimensions change.

If you're lazy-loading images or implementing responsive images with different sizes at different breakpoints, the markers automatically adjust when the image size changes.

### Cross-Device Consistency

Because coordinates are stored as percentages in the database, a discussion created on a desktop computer appears at the exact same relative location when viewed on a tablet or phone. Users can collaborate across devices without any positioning inconsistencies.

This works bidirectionally. A discussion created by tapping a specific spot on a mobile device appears at the same relative position when viewed on a large desktop monitor.

### Viewport Considerations

The widget calculates percentages relative to the image element itself, not the viewport. This means scrolling the page or changing the browser window size doesn't affect marker positions. Markers remain anchored to their locations on the image regardless of viewport changes.

### Future-Proofing Content

The percentage-based approach makes your image discussions resilient to changes in layout, design, or device ecosystem. As new screen sizes and devices emerge, the existing discussions will continue to display correctly without requiring any updates or migrations.

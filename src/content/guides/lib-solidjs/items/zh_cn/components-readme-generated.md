来自 `fastcomments-react` 的每个小部件都可使用相同的名称：

| 组件 | Handle 类型 | 加载的嵌入 |
| --- | --- | --- |
| `FastCommentsCommentWidget` | `FastCommentsCommentWidgetHandle` | 旗舰实时评论小部件 |
| `FastCommentsCommentCountWidget` | `FastCommentsCommentCountWidgetHandle` | 内联评论计数徽章 |
| `FastCommentsLiveChatWidget` | `FastCommentsLiveChatWidgetHandle` | 流式实时聊天小部件 |
| `FastCommentsCollabChatWidget` | `FastCommentsCollabChatWidgetHandle` | 基于文本锚点的协作聊天 |
| `FastCommentsImageChatWidget` | `FastCommentsImageChatWidgetHandle` | 基于区域的图像评论 |
| `FastCommentsRecentCommentsWidget` | `FastCommentsRecentCommentsWidgetHandle` | 最新评论提要 |
| `FastCommentsRecentDiscussionsWidget` | `FastCommentsRecentDiscussionsWidgetHandle` | 最近讨论提要 |
| `FastCommentsReviewsSummaryWidget` | `FastCommentsReviewsSummaryWidgetHandle` | 星级评分汇总 |
| `FastCommentsTopPagesWidget` | `FastCommentsTopPagesWidgetHandle` | 评论最多页面排行榜 |
| `FastCommentsUserActivityFeedWidget` | `FastCommentsUserActivityFeedWidgetHandle` | 每用户的活动时间线 |

### 挂载到现有元素的组件

`FastCommentsCollabChatWidget` 和 `FastCommentsImageChatWidget` 会挂载到调用者提供的元素。传入一个 `targetRef` 访问器，该访问器在元素挂载后返回该元素：

```tsx
import { FastCommentsImageChatWidget } from 'fastcomments-solidjs';

export default function ImageChat() {
  let img: HTMLImageElement | undefined;
  return (
    <>
      <img ref={img} src="/screenshot.png" alt="" />
      <FastCommentsImageChatWidget
        tenantId="demo"
        urlId="my-image"
        targetRef={() => img}
      />
    </>
  );
}
```

### 区域

传入 `region="eu"` 以将小部件流量通过欧盟集群路由。
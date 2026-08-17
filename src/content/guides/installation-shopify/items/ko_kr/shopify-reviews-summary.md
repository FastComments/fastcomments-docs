The **FastComments - Reviews Summary** 블록은 페이지에 대한 집계된 별점과 리뷰 분류를 표시합니다. 표준 리뷰 레이아웃을 위해 제품 템플릿에 **FastComments** 블록과 함께 사용하세요: 상단에 요약, 그 아래에 리뷰 양식 및 리뷰가 표시됩니다.

### 전제 조건: Ratings & Reviews 설정

Reviews Summary 블록은 스토어에 설정한 평점 질문을 표시합니다. 먼저 이를 설정하세요:

1. Shopify 관리자에서 FastComments 앱을 엽니다.
2. **Ratings & Reviews Helper** 타일을 클릭합니다 (또는 [Ratings & Reviews Helper](https://fastcomments.com/auth/my-account/ratings-reviews-helper?source=shopify) 를 직접 엽니다).
3. 각 리뷰어가 답변하도록 원하는 질문을 추가합니다 (전체 별점, "핏은 어땠나요" 등).

질문이 설정되지 않으면 요약 블록에 집계할 내용이 없습니다.

### 블록 추가

1. Shopify 테마 편집기를 엽니다.
2. **Product** 템플릿을 엽니다 (또는 요약을 표시하고 싶은 페이지 템플릿).
3. 페이지 섹션 상단, **FastComments** 블록이 위치할 위쪽에 있는 **Add block** 을 클릭합니다.
4. **Apps** 아래에서 **FastComments - Reviews Summary** 를 선택합니다.
5. 아직 추가하지 않았다면 같은 페이지 아래쪽에 **FastComments** 블록을 추가하여 방문자가 리뷰를 남길 수 있게 합니다.
6. **Save** 를 클릭합니다.

### 설정

| 설정 | 설명 | 기본값 |
|---|---|---|
| Tenant ID (옵션) | 요약이 읽어올 FastComments 테넌트를 재정의합니다. 비워 두면 스토어에 자동으로 구성된 테넌트를 사용합니다. | (blank) |
| Custom URL ID | 요약이 집계할 페이지 식별자를 재정의합니다. 요약이 해당 FastComments 블록과 다른 페이지에 있을 때 사용합니다. | (auto-detected) |

### 요약이 리뷰와 일치하는 방식

Reviews Summary 블록은 **FastComments** 블록과 동일한 자동 감지 로직을 사용합니다:

- 제품 템플릿: `shopify-product-{product.id}`
- 블로그 게시물 템플릿: `shopify-article-{article.id}`
- 다른 템플릿: 요청 경로

일반 제품 페이지에서는 요약과 댓글 스레드가 자동으로 URL ID를 공유하므로 별도의 설정이 필요하지 않습니다.

### 팁

- 요약은 읽기 전용입니다. 리뷰를 수집하려면 같은 페이지에 **FastComments** 블록이 필요합니다.
- 리뷰를 수집한 후 Ratings & Reviews Helper에서 평점 질문을 변경하면, 요약은 새로운 질문 세트에 따라 다시 계산됩니다.

---
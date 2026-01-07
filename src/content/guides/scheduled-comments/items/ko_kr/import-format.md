### 형식 예제

예약 댓글 CSV는 다음과 같습니다:

|ID |Parent ID|URL ID       |URL|Name         |Avatar|Comment                              | Hours | Minutes | Seconds |
|---|---------|-------------|---|-------------|------|-------------------------------------|-------|---------|---------|
|1  |         |scheduledtest|   |Test Person 1|      |Hello!                               | 0     | 0       | 3       |
|2  |1        |scheduledtest|   |Test Person 2|      |How are you?                         | 0     | 0       | 10      |
|3  |2        |scheduledtest|   |Test Person 3|      |Leave them alone                     | 0     | 1       | 30      |
|4  |         |scheduledtest|   |Test Person 4|      |Does anyone have a cute cat to share?| 1     | 10      | 0       |


### 형식 세부 정보

예약 댓글 CSV 파일은 다음 열을 지원합니다:

- ID
- Parent ID
- URL ID
- URL
- Name
- Avatar
- Comment
- Hours
- Minutes
- Seconds

다음 열은 **필수**입니다:

- ID
- URL ID
- Name
- Comment
- Hours
- Minutes
- Seconds

자동화된 중첩 답글을 지원하려면 Parent ID 열이 필요합니다.

### 형식 작동 방식

가져오기 형식은 다음과 같이 작동합니다:

1. 게시하려는 각 댓글에 대해 CSV에 행을 정의합니다.
2. 댓글은 지정된 지연(시간 + 분 + 초) 후에 게시됩니다.
   1. 수동으로 예약된 가져오기의 경우 지연은 가져오기가 완료된 후 UI에서 "play"를 누를 때를 기준으로 합니다 - **가져오기가 시작될 때가 아닙니다**.
   2. 자동 예약 가져오기의 경우 지연은 페이지 로드 시점부터입니다.
3. ID를 정의해야 합니다. 1, 2, 3, 4, 5와 같은 증가하는 식별자를 사용할 수 있습니다.
4. 중첩 답글을 사용하려면 `Parent ID` 열 값을 다른 댓글의 `ID`로 설정하면 됩니다.
5. `Comment` 필드는 FastComments가 댓글 위젯에서 텍스트 스타일링 및 이미지 추가를 위해 지원하는 것과 동일한 구문을 지원합니다.
6. `Avatar` 필드를 사용하는 경우 공개적으로 접근 가능한 이미지여야 합니다. 당사 CDN으로 복사되어 제공됩니다.
7. 재생 후 또는 재생이 중지되면 모든 댓글을 삭제할 수 있습니다.
8. 삭제는 실시간으로 이루어지므로 동일한 예약 가져오기를 반복해서 재사용할 수 있습니다.

### 예제

[예제 CSV 파일은 여기에 있습니다](/csv/fastcomments-scheduled-comments-example.csv).

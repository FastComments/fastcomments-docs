---
По подразумевању, FastComments ће дозволити корисницима да обришу своје коментаре.

Међутим, могуће је спречити ово.

На страници за прилагођавање виџета, погледајте опцију "Онемогући брисање".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-delete'; selector = '.disable-commenter-comment-delete'; title='Disable Comment Deleting' app-screenshot-end]

- Ово утиче само на редовне коментаторе и не утиче на модераторе или администраторе, који ће и даље моћи да бришу.
- Ово ће такође утицати на API интеграције када се проследи `contextUserId`. 

---
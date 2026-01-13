---
Zoals elk gedistribueerd systeem heeft FastComments een manier nodig om bronnen en procedures te vergrendelen. Deze vergrendelingen kunnen worden bewaakt via het `/locks-in-progress` eindpunt.

[Bijvoorbeeld, hier is het eindpunt op onze US shard](https://fastcomments.com/locks-in-progress).

Dit kan nuttig zijn om te zien waarom het systeem vastloopt of onder belasting staat. Als een SRE bijvoorbeeld wil zien waarom het systeem een hoge CPU-belasting ervaart, kunnen ze
dit eindpunt raadplegen om de naam van de cron te achterhalen die zich misdraagt.

---
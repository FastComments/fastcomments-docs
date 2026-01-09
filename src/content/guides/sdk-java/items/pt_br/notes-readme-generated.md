### IDs de Broadcast

Você verá que deve passar um `broadcastId` em algumas chamadas de API. Quando você receber eventos, esse ID será retornado, então você saberá ignorar o evento se planejar aplicar alterações otimisticamente no cliente
(o que você provavelmente desejará fazer, pois oferece a melhor experiência). Passe um UUID aqui. O ID deve ser suficientemente único para não ocorrer duas vezes em uma sessão do navegador.
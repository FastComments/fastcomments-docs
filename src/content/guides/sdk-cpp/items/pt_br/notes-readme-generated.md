### IDs de Broadcast

Você verá que deve passar um `broadcastId` em algumas chamadas de API. Quando você receber eventos, receberá esse ID de volta, então saberá ignorar o evento se planejar aplicar alterações otimisticamente no cliente
(o que você provavelmente vai querer fazer, já que oferece a melhor experiência). Passe um UUID aqui. O ID deve ser único o suficiente para não ocorrer duas vezes em uma sessão do navegador.

### SSO (Login Único)

Para exemplos de SSO, veja abaixo.
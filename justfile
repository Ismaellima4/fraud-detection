# Sobe o ambiente (api1, api2, nginx) sem rebuild
up:
    docker-compose -f containerization/docker-compose.yml up -d

# Sobe o ambiente com rebuild das imagens
build:
    docker-compose -f containerization/docker-compose.yml up -d --build

# Para todos os serviços
down:
    docker-compose -f containerization/docker-compose.yml down

# Executa os testes de carga com k6 usando a rede host
test:
    docker run --rm -i --network host \
      -v $(pwd):/home/k6 \
      -w /home/k6 \
      --user $(id -u):$(id -g) \
      grafana/k6 run test/test.js

# Sobe o ambiente e executa os testes
up-test: up
    just test

# Sobe o ambiente com build e executa os testes
build-test: build
    just test

# Publica a imagem no GHCR
publish-ghcr:
    ./scripts/publish-ghcr.sh

#!/bin/bash

# Esse script tem como função exportar as pastas e arquivos de uma imagem alpine. Assim, é possível executar o nayr.

sudo umount containers/teste 2>/dev/null

sudo rm -rf ../images/base ../overlays/teste ../containers/teste
mkdir -p ../images/base ../overlays/ ../containers/

docker run --name alpine-temp alpine true
docker export alpine-temp > alpine-rootfs.tar
docker rm alpine-temp

sudo tar -xf alpine-rootfs.tar -C ../images/base/

rm alpine-rootfs.tar
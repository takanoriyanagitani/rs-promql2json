#!/bin/bash

wsm="./target/wasm32-wasip1/release-wasi/rs-promql2json.wasm"

qlsz_max=65536

ex1(){
  echo 'avg by (instance) (rate(cpu_seconds_tot{mode="idle"}[5m]))' |
    wazero run -env ENV_PROMQL_SIZE_MAX=$qlsz_max "${wsm}" |
    dasel --in=json --out=yaml |
    bat --language=yaml
}

ex2(){
  echo '100.0 * (mem_available / mem_total)' |
    wazero run -env ENV_PROMQL_SIZE_MAX=$qlsz_max "${wsm}" |
    dasel --in=json --out=yaml |
    bat --language=yaml
}

ex3(){
  echo 'fs_avail_bytes{mount="/"}' |
    wazero run -env ENV_PROMQL_SIZE_MAX=$qlsz_max "${wsm}" |
    dasel --in=json --out=yaml |
    bat --language=yaml
}

ex4(){
  echo 'rate(net_transmit_bytes_tot{dev="eth0"}[5m])' |
    wazero run -env ENV_PROMQL_SIZE_MAX=$qlsz_max "${wsm}" |
    dasel --in=json --out=yaml |
    bat --language=yaml
}

ex5(){
  echo 'load_avg_1' |
    wazero run -env ENV_PROMQL_SIZE_MAX=$qlsz_max "${wsm}" |
    dasel --in=json --out=yaml |
    bat --language=yaml
}

#ex1
#ex2
#ex3
ex4
#ex5

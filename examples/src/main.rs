nok::korean! {
    외부 크레이트 nok;

    사용 std::collections::사전 다음_처럼 Dic;

    열거 EN {
        Oh,
        Hi,
    }

    특성 SomeTrait {
        함수 안녕1(&self, val: 문자열, value: 문자열);
        함수 안녕2(&self, val: 문자열) -> 결과<옵션<&'static 문자열>, 문자열>;
    }

    정적 가변 DICT: 옵션<Dic<문자열, 문자열>> = 없음;

    구조 Something;

    구현 SomeTrait 다음을_위해 Something {
        함수 안녕1(&self, val: 문자열, value: 문자열) {
            여기서 dico = 위험 {
                DICT.get_or_insert_with(기본::default)
            };
            dico.넣기(val, value);
        }
        함수 안녕2(&self, val: 문자열) -> 결과<옵션<&'static 문자열>, 문자열> {
            만약 여기서 있음(dico) = 위험 { DICT.as_ref() } {
                오케이(dico.얻기(&val))
            } 아니면 {
                실패("실패함".into())
            }
        }
    }

    공용(크레이트) 함수 some_thing(i: u32) -> 옵션<결과<u32, 문자열>> {
        만약 i % 2 == 1 {
            만약 i == 42 {
                있음(실패(문자열::다음에서("안녕")))
            } 아니면 {
                있음(오케이(33))
            }
        } 아니면 {
            없음
        }
    }

    비동기 함수 example() {
        구현안됨!()
    }

    함수 그냥() {
        여기서 가변 x = 31;

        매치 x {
            42 => {
                출력!("42 네요.")
            }
            _ => 출력!("여기로 옴")
        }

        다음을_위해 i 안 0..10 {
            여기서 val = 무한 {
                탈출 i;
            };

            조건_반복 아니 x < val {
                x += 1;
            }

            x = 만약 여기서 있음(ok) = some_thing(i) {
                ok.꺼내기()
            } 아니면 {
                12
            };
        }

    }

    #[허용(접근_불가_코드)]
    함수 접근_불가() {
        안돼!("안녕히 가세요.");
        종료!("안녕히 가세요.");
        패닉!("안녕히 가세요.");
    }

    // this is main function
    함수 메인() {
        출력!("{} 오케이 ㅋㅋ", "안녕");
        그냥();
        여기서 test: 문자열 = "Hello World".to_string();
        출력!("test: {}", test);

        여기서 가변 total_sum = 0;
        여기서 result1 = 'main: 무한 {
            다음을_위해 i 안 0..10000 {
                total_sum += i;
                만약 i == 100 {
                    탈출 'main total_sum * 2; // break main and result1 will be total_sum * 2
                }
            }
        };
        출력!("총 합: {}", total_sum);  // 5050
        안돼!("안녕히 가세요.");
    }
}

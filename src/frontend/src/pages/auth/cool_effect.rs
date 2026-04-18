use leptos::prelude::*;

#[component]
pub fn PerlinNoiseBg() -> impl IntoView {
    view! {
        <div
            id="perlin-bg"
            style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; background-color: white;"
        />
        <div
            id="perlin-skeleton"
            class=crate::cls!("absolute top-0 left-0 w-full h-full bg-gradient-to-br from-background via-card to-background animate-pulse")
        />
        <script src="https://fastly.jsdelivr.net/npm/echarts@5/dist/echarts.min.js"></script>
        <script>
            {build_perlin_inline_script()}
        </script>
    }
}

fn build_perlin_inline_script() -> &'static str {
    r#"
    document.addEventListener('DOMContentLoaded', function() {
        (function() {
            const container = document.getElementById('perlin-bg');
            const skeleton = document.getElementById('perlin-skeleton');
            if (!container) return;

            const myChart = echarts.init(container);
            const noise = getNoiseHelper();

            let config = {
                frequency: 500,
                offsetX: 10,
                offsetY: 100,
                minSize: 40,
                maxSize: 80,
                duration: 5000,
                color0: '#d4bc8b',
                color1: '#474237',
                backgroundColor: '#141412',
                onChange() {
                    myChart.setOption({
                        backgroundColor: config.backgroundColor,
                        graphic: {
                            elements: createElements()
                        }
                    });
                }
            };

            noise.seed(Math.random());

            function createElements() {
                const elements = [];
                for (let x = 20; x < myChart.getWidth(); x += 40) {
                    for (let y = 20; y < myChart.getHeight(); y += 40) {
                        const rand = noise.perlin2(
                            x / config.frequency + config.offsetX,
                            y / config.frequency + config.offsetY
                        );
                        elements.push({
                            type: 'circle',
                            x,
                            y,
                            style: {
                                fill: config.color1
                            },
                            shape: {
                                r: config.maxSize
                            },
                            keyframeAnimation: {
                                duration: config.duration,
                                loop: true,
                                delay: (rand - 1) * 4000,
                                keyframes: [
                                    {
                                        percent: 0.5,
                                        easing: 'sinusoidalInOut',
                                        style: {
                                            fill: config.color0
                                        },
                                        scaleX: config.minSize / config.maxSize,
                                        scaleY: config.minSize / config.maxSize
                                    },
                                    {
                                        percent: 1,
                                        easing: 'sinusoidalInOut',
                                        style: {
                                            fill: config.color1
                                        },
                                        scaleX: 1,
                                        scaleY: 1
                                    }
                                ]
                            }
                        });
                    }
                }
                return elements;
            }

            const option = {
                backgroundColor: config.backgroundColor,
                graphic: {
                    elements: createElements()
                }
            };

            myChart.setOption(option);

            // Hide skeleton once initialized
            if (skeleton) skeleton.style.display = 'none';

            window.addEventListener('resize', () => {
                myChart.resize();
                myChart.setOption(option);
            });

            // Perlin noise helper
            function getNoiseHelper() {
                class Grad {
                    constructor(x, y, z) {
                        this.x = x;
                        this.y = y;
                        this.z = z;
                    }
                    dot2(x, y) {
                        return this.x * x + this.y * y;
                    }
                    dot3(x, y, z) {
                        return this.x * x + this.y * y + this.z * z;
                    }
                }

                const grad3 = [
                    new Grad(1, 1, 0), new Grad(-1, 1, 0), new Grad(1, -1, 0), new Grad(-1, -1, 0),
                    new Grad(1, 0, 1), new Grad(-1, 0, 1), new Grad(1, 0, -1), new Grad(-1, 0, -1),
                    new Grad(0, 1, 1), new Grad(0, -1, 1), new Grad(0, 1, -1), new Grad(0, -1, -1)
                ];

                const p = [151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180];

                let perm = new Array(512);
                let gradP = new Array(512);

                function seed(seed) {
                    if (seed > 0 && seed < 1) {
                        seed *= 65536;
                    }
                    seed = Math.floor(seed);
                    if (seed < 256) {
                        seed |= seed << 8;
                    }
                    for (let i = 0; i < 256; i++) {
                        let v;
                        if (i & 1) {
                            v = p[i] ^ (seed & 255);
                        } else {
                            v = p[i] ^ ((seed >> 8) & 255);
                        }
                        perm[i] = perm[i + 256] = v;
                        gradP[i] = gradP[i + 256] = grad3[v % 12];
                    }
                }

                seed(0);

                function fade(t) {
                    return t * t * t * (t * (t * 6 - 15) + 10);
                }

                function lerp(a, b, t) {
                    return (1 - t) * a + t * b;
                }

                function perlin2(x, y) {
                    let X = Math.floor(x), Y = Math.floor(y);
                    x = x - X;
                    y = y - Y;
                    X = X & 255;
                    Y = Y & 255;

                    let n00 = gradP[X + perm[Y]].dot2(x, y);
                    let n01 = gradP[X + perm[Y + 1]].dot2(x, y - 1);
                    let n10 = gradP[X + 1 + perm[Y]].dot2(x - 1, y);
                    let n11 = gradP[X + 1 + perm[Y + 1]].dot2(x - 1, y - 1);

                    let u = fade(x);
                    return lerp(lerp(n00, n10, u), lerp(n01, n11, u), fade(y));
                }

                return { seed, perlin2 };
            }
        })();
    });
    "#
}


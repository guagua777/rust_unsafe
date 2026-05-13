#include <cstdio>
#include <cmath>
#include <xmmintrin.h>

#define N 1024
#define EPSILON 1e-3f

static float A[N][N];
static float x[N];
static float w[N];
static float test2[N];

void routine2(float alpha, float beta) {
    for (unsigned i = 0; i < N; i++) {
        float dot = 0.f;
        for (unsigned j = 0; j < N; j++)
            dot += A[i][j] * x[j];
        w[i] += alpha * dot - N * beta;
    }
}

void routine2_vec(float alpha, float beta) {
    __m128 alpha4 = _mm_set1_ps(alpha);
    __m128 beta_n = _mm_set1_ps(N * beta);
    for (unsigned i = 0; i < N; i += 4) {
        __m128 dot4_1 = _mm_setzero_ps();
        // ... (其他向量化代码)
        __m128 wi = _mm_loadu_ps(&w[i]);
        wi = _mm_add_ps(wi, _mm_sub_ps(_mm_mul_ps(dot4_1, alpha4), beta_n));
        _mm_storeu_ps(&w[i], wi);
    }
}


int routine2_test(float alpha, float beta);

int main() {
    routine2(1.0f, 0.5f);
    routine2_vec(1.0f, 0.5f);
    return routine2_test(1.0f, 0.5f);
}

int routine2_test(float alpha, float beta) {
    // 初始化并计算test2数组
    for (unsigned i = 0; i < N; i++) {
        float dot = 0.f;
        for (unsigned j = 0; j < N; j++)
            dot += A[i][j] * x[j];
        test2[i] = alpha * dot - N * beta;
    }
    // 比较w和test2的结果
    for (unsigned j = 0; j < N; j++) {
        if (fabs(w[j] - test2[j]) > EPSILON) {
            printf("\n The result of w[%d] is not equal to test2[%d]  \n", j, j);
            return 1;
        }
    }
    return 0;
}


# Алгоритмы решения 0-1 Knapsack Problem

### 1. **Полный перебор через битовые маски**
- **Асимптотика**:  
  - Время: `O(2^n * n)`  
  - Память: `O(1)`  
- **Приближённый или точный**: Точный.  
- **Описание особенностей**:  
  Используются все возможные подмножества предметов, что можно закодировать битовыми масками длины `n`. Для каждой маски проверяется, можно ли уместить выбранные предметы в рюкзак, и вычисляется их стоимость. Подходит для небольших значений `n` (до 20-25).
- **Relevant Links**:
  - [GeeksForGeeks: Bitmasking approach to 0/1 Knapsack](https://www.geeksforgeeks.org/bitmasking-dynamic-programming-set-2-knapsack/)
  - [CP Algorithms: Bitmask operations](https://cp-algorithms.com/algebra/all-submasks.html)

---

### 2. **Полный перебор через рекурсию**
- **Асимптотика**:  
  - Время: `O(2^n)`  
  - Память: `O(n)` (глубина стека вызовов).  
- **Приближённый или точный**: Точный.  
- **Описание особенностей**:  
  Рекурсивно рассматриваются два случая для каждого предмета: включить его в рюкзак или пропустить.
- **Relevant Links**:
  - [GeeksForGeeks: Recursive approach for Knapsack](https://www.geeksforgeeks.org/0-1-knapsack-problem-recursive-implementation/)
  - [Stanford CS: Recursive Backtracking](https://web.stanford.edu/class/cs106b/lectures/05-backtracking/)

---

### 3. **Meet in the middle**
- **Асимптотика**:  
  - Время: `O(2^(n/2) * n)`  
  - Память: `O(2^(n/2))`  
- **Приближённый или точный**: Точный.  
- **Описание особенностей**:  
  Делит набор предметов на две половины. Для каждой половины генерируются все подмножества с их весом и стоимостью. Затем результаты двух половин объединяются с помощью двоичного поиска. Это существенно ускоряет решение для `n` порядка 40.
- **Relevant Links**:
  - [CP Algorithms: Meet in the middle technique](https://cp-algorithms.com/others/meet-in-the-middle.html)
  - [TopCoder: Meet in the Middle](https://www.topcoder.com/thrive/articles/Meet%20in%20the%20Middle)

---

### 4. **Динамическое программирование `O(nW)`**
- **Асимптотика**:  
  - Время: `O(nW)`  
  - Память: `O(W)` (можно оптимизировать с помощью одномерного массива).  
- **Приближённый или точный**: Точный.  
- **Описание особенностей**:  
  Создаётся таблица, где `dp[i][w]` представляет максимальную стоимость при использовании первых `i` предметов и весе `w`. На каждой итерации рассматриваются два случая: включить или пропустить текущий предмет. Хорошо работает для небольших `W`.
- **Relevant Links**:
  - [GeeksForGeeks: DP approach to Knapsack](https://www.geeksforgeeks.org/0-1-knapsack-problem-dp-10/)
  - [MIT OpenCourseWare: Dynamic Programming](https://ocw.mit.edu/courses/6-006-introduction-to-algorithms-fall-2011/resources/lecture-19-dynamic-programming-i-fibonacci-shortest-paths/)

---

### 5. **Ленивая динамика**
- **Асимптотика**:  
  - Время: Зависит от количества достижимых состояний, часто лучше, чем `O(nW)`.  
  - Память: Зависит от количества достижимых состояний (реже используется вся таблица `O(nW)`).  
- **Приближённый или точный**: Точный.  
- **Описание особенностей**:  
  Вместо того чтобы вычислять все состояния, вычисляются только те, которые возможны с учётом ограничений (веса). Это снижает избыточные вычисления и работает быстрее для некоторых задач.
- **Relevant Links**:
  - [Algorithmica: Lazy Dynamic Programming](https://algorithmica.org/en/lazy-segment-tree)
  - [CodeForces: Lazy Dynamic Programming Blog](https://codeforces.com/blog/entry/78733)

---

### 6. **Жадный алгоритм**
- **Асимптотика**:  
  - Время: `O(n log n)` (на сортировку).  
  - Память: `O(n)`.  
- **Приближённый или точный**: Приближённый (для дробного рюкзака точный).  
- **Описание особенностей**:  
  Предметы сортируются по убыванию отношения ценности к весу `v[i]/w[i]`. Включаются в рюкзак до исчерпания веса. Эффективен для дробного рюкзака, но для 0-1 версии может быть далёким от оптимального.
- **Relevant Links**:
  - [GeeksForGeeks: Greedy approach for Fractional Knapsack](https://www.geeksforgeeks.org/fractional-knapsack-problem/)
  - [Stanford CS: Greedy Algorithms](https://web.stanford.edu/class/archive/cs/cs161/cs161.1138/lectures/13/Small13.pdf)

---

### 7. **FPTAS (Fully Polynomial-Time Approximation Scheme)**
- **Асимптотика**:  
  - Время: `O(n^2 / ε)`.  
  - Память: `O(n / ε)`.  
- **Приближённый или точный**: Приближённый (с гарантией близости к оптимальному).  
- **Описание особенностей**:  
  Снижает точность значений предметов (стоимостей), округляя их до меньшего числа разрядов с использованием коэффициента `ε`. Использует динамическое программирование для решения задачи с уменьшенной точностью. Позволяет находить решение с точностью до `(1 + ε)`.
- **Relevant Links**:
  - [CMU: FPTAS for Knapsack](https://www.cs.cmu.edu/~ckingsf/bioinfo-lectures/fptas.pdf)
  - [UC Davis: Approximation Algorithms](https://web.cs.ucdavis.edu/~bai/ECS222/approximation.pdf)

---

### 8. **Метод ветвей и границ**
- **Асимптотика**:  
  - Время: Зависит от конкретных ограничений и порядка ветвления, в худшем случае `O(2^n)`.  
  - Память: `O(n)` (глубина стека).  
- **Приближённый или точный**: Точный.  
- **Описание особенностей**:  
  Применяется рекурсивное дерево решений с отсечением нерелевантных ветвей, используя эвристики (например, оценку максимальной стоимости при текущем весе). Позволяет существенно ускорить решение за счёт отсечения очевидно неоптимальных решений.
- **Relevant Links**:
  - [GeeksForGeeks: Branch and Bound](https://www.geeksforgeeks.org/branch-and-bound-algorithm/)
  - [Cornell University: Branch and Bound Methods](https://people.orie.cornell.edu/dpw/orie6300/Lectures/lec7.pdf)


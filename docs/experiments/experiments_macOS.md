## Experiment Part 2

### 1. Предположение
- Используемые алгоритмы, кроме Greedy, имеют 100% точность.
- Greedy отрабатывает быстрее всех алгоритмов.
- Ленивая динамика и рекурсия имеют почти идентичный результат по времени.
- Полный перебор с битовыми масками имеет худший результат по времени.

### 2. Алгоритмы
- Bitmask
- Lazy Dynamic Programming
- Recursion
- Dynamic Programming
- Greedy

### 3. Данные
Эксперимент проведен на рандомизированных рюкзаках, составленных из 10/15/20 предметов по 20 сэмплов.

### 4. Проведение эксперимента
Перед запуском тестов:
- Закрыты ненужные приложения и процессы, в том числе фоновые.
- Очищен кэш памяти с помощью команды:
  ```bash
  sudo sync; echo 3 | sudo tee /proc/sys/vm/drop_caches
  ```
- Для каждого теста сделан прогрев внутри бенчмарка в течение 1 секунды.

### 5. Метрики производительности
- Точность алгоритма измерена как процентное соотношение ответов, являющихся максимальными для каждого рюкзака, удовлетворяющих условию задачи.
- Время измерено в миллисекундах, сообщаются четыре метрики. Цитата из либы:
>The second line shows confidence intervals on the mean and standard deviation of the per-iteration times (calculated naively). If std. >dev. is large compared to the time values from above, the benchmarks are noisy. You may need to change your benchmark to reduce the >noise.
>The median/med. abs. dev. line is similar to the mean/std. dev. line, except that it uses the median and median absolute deviation. As >with the std. dev., if the med. abs. dev. is large, this indicates the benchmarks are noisy.



### 6. Результаты

#### 10 items
<pre>
</pre>


#### 15 items
<pre>
</pre>


#### 20 items
<pre>
</pre>

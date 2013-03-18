-- WORK IN PROGRESS, NOT YET FINISHED!

CREATE OR REPLACE PROCEDURE Eratosthenes AS
  primes dbms_sql.number_table;
  max_number NUMBER := 120;
  next_potential_prime NUMBER := 2;
  loop_tmp NUMBER;
  loop_step NUMBER;
BEGIN

  -- initialize table
  FOR i IN 2..max_number LOOP
    primes(i) := 1;
  END LOOP;

  -- main loop, mark non-primes with 0
  FOR i IN 2..sqrt(max_number) LOOP
    IF primes(i) = 1 THEN
      loop_tmp := i * i;
      loop_step := 1;
      WHILE loop_tmp <= max_number LOOP
        primes(loop_tmp) := 0;
        loop_tmp := loop_tmp + loop_step * i;
      END LOOP;
    END IF;
  END LOOP;

  -- output result
  FOR i IN 2..max_number LOOP
    IF primes(i) = 1 THEN
      dbms_output.put(i || ' ');
    END IF;
  END LOOP;
  dbms_output.put_line('');

END Eratosthenes;

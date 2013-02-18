-- WORK IN PROGRESS, NOT YET FINISHED!

create or replace procedure erastothenes as
  primes dbms_sql.number_table;
  max_number number := 120;
  next_potential_prime number := 2;
begin

  -- initialize table
  for i in 2..max_number loop
    primes(i) := 1;
  end loop;

  -- main loop
  for i in 2..max_number loop
    if primes(i) = 1 then
      null;
      -- TODO continue here
    end if;
  end loop;

  for i in 2..max_number loop
    dbms_output.put_line('Prime ' || i || ': ' || primes(i));
  end loop;

end erastothenes;

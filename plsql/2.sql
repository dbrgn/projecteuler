create or replace procedure fibonacci as

  next_fib number;
  n number := 1;
  even_sum number := 0;

  -- return the nth fibonacci number
  function fibonacci (n number) return number is
  begin
    if n > 2 then
      return fibonacci(n - 1) + fibonacci(n - 2);
    else
      return n + 1;
    end if;
  end;

begin

  -- loop over fibonacci numbers until it exceeds 4 mio
  loop
    next_fib := fibonacci(n);
    exit when next_fib > 4000000;
    dbms_output.put_line('Next fibonacci: ' || next_fib);
    if mod(next_fib, 2) = 0 then
      even_sum := even_sum + next_fib;
    end if;
    n := n + 1;
  end loop;

  dbms_output.put_line('Sum of even fibonacci numbers: ' || even_sum);

end fibonacci;

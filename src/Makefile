CC = g++

FLAGS = -g

LIBS = -lpthread

OBJS = count_task_cpp.o  factory.o  file_operate.o  public.o  thread_pool.o \
		main.o  count_task_shell.o

main:$(OBJS)
	$(CC) $(FLAGS) -o counter $(OBJS) $(LIBS)

clean:
	rm -f $(OBJS) counter
#include <iostream>
#include "file_operate.h"
#include "factory.h"
#include "thread_pool.h"




void run(string path)
{	
	CFileOperate fileoperate(new CThreadPool(MAX_THREAD_NUM));
	fileoperate.readFile(path);

	while (1)
	{
		if (fileoperate._pool->getTaskSize() == 0)
		{
			if (fileoperate._pool->StopAll() == -1)
			{
				exit(0);
			}
		}
		sleep(1);
	}

}

int main(int argc, char* argv[])
{
	if (argc != 2)
	{
		fprintf(stderr, "Usage: %s <pathname>\n", argv[0]);
		exit(0);
	}
	run(argv[1]);

	return 0;
}

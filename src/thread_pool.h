#ifndef __THREAD_H  
#define __THREAD_H  

#include <vector>  
#include <string>  
#include <pthread.h>  
#include <cstdio>
#include <unistd.h>
#include  <cstdlib>
#include "count_task.h"
#define MAX_THREAD_NUM 10
using namespace std;




class CThreadPool
{
private:
	static  vector<CCountTask*> _vecTaskList;     /** �����б� */
	static  bool _shutdown;                    /** �߳��˳���־ */
	int     _iThreadNum;                     /** �̳߳����������߳��� */
	pthread_t   *_pthread_id;

	static pthread_mutex_t _pthreadMutex;    /** �߳�ͬ���� */
	static pthread_cond_t _pthreadCond;      /** �߳�ͬ������������ */

protected:
	static void* ThreadFunc(void * threadData); /** ���̵߳��̻߳ص����� */
	static int MoveToIdle(pthread_t tid);       /** �߳�ִ�н����󣬰��Լ����뵽�����߳��� */
	static int MoveToBusy(pthread_t tid);       /** ���뵽æµ�߳���ȥ */

	int Create();          /** �����̳߳��е��߳� */

public:
	CThreadPool(int threadNum = 10);
	int AddTask(CCountTask *task);      /** ��������ӵ���������� */
	int StopAll();                 /** ʹ�̳߳��е��߳��˳� */
	int getTaskSize();             /** ��ȡ��ǰ��������е������� */
};

#endif  

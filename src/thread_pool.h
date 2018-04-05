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
	static  vector<CCountTask*> _vecTaskList;     /** 任务列表 */
	static  bool _shutdown;                    /** 线程退出标志 */
	int     _iThreadNum;                     /** 线程池中启动的线程数 */
	pthread_t   *_pthread_id;

	static pthread_mutex_t _pthreadMutex;    /** 线程同步锁 */
	static pthread_cond_t _pthreadCond;      /** 线程同步的条件变量 */

protected:
	static void* ThreadFunc(void * threadData); /** 新线程的线程回调函数 */
	static int MoveToIdle(pthread_t tid);       /** 线程执行结束后，把自己放入到空闲线程中 */
	static int MoveToBusy(pthread_t tid);       /** 移入到忙碌线程中去 */

	int Create();          /** 创建线程池中的线程 */

public:
	CThreadPool(int threadNum = 10);
	int AddTask(CCountTask *task);      /** 把任务添加到任务队列中 */
	int StopAll();                 /** 使线程池中的线程退出 */
	int getTaskSize();             /** 获取当前任务队列中的任务数 */
};

#endif  
